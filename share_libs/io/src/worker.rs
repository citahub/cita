// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use IoHandler;
use crossbeam::sync::chase_lev;
use service::{HandlerId, IoChannel, IoContext};
use std::cell::Cell;

use std::sync::{Condvar as SCondvar, Mutex as SMutex};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::thread::{self, JoinHandle};

const STACK_SIZE: usize = 16 * 1024 * 1024;

thread_local! {
    /// Stack size
    /// Should be modified if it is changed in Rust since it is no way
    /// to know or get it
    pub static LOCAL_STACK_SIZE: Cell<usize> = Cell::new(::std::env::var("RUST_MIN_STACK").ok().and_then(|s| s.parse().ok()).unwrap_or(2 * 1024 * 1024));
}

pub enum WorkType<Message> {
    Readable,
    Writable,
    Hup,
    Timeout,
    Message(Message),
}

pub struct Work<Message> {
    pub work_type: WorkType<Message>,
    pub token: usize,
    pub handler_id: HandlerId,
    pub handler: Arc<IoHandler<Message>>,
}

/// An IO worker thread
/// Sorts them ready for blockchain insertion.
pub struct Worker {
    thread: Option<JoinHandle<()>>,
    wait: Arc<SCondvar>,
    deleting: Arc<AtomicBool>,
    wait_mutex: Arc<SMutex<()>>,
}

impl Worker {
    /// Creates a new worker instance.
    pub fn new<Message>(index: usize, stealer: chase_lev::Stealer<Work<Message>>, channel: IoChannel<Message>, wait: Arc<SCondvar>, wait_mutex: Arc<SMutex<()>>) -> Worker
    where
        Message: Send + Sync + Clone + 'static,
    {
        let deleting = Arc::new(AtomicBool::new(false));
        let mut worker = Worker {
            thread: None,
            wait: wait.clone(),
            deleting: deleting.clone(),
            wait_mutex: wait_mutex.clone(),
        };
        worker.thread = Some(thread::Builder::new()
                                 .stack_size(STACK_SIZE)
                                 .name(format!("IO Worker #{}", index))
                                 .spawn(move || {
                                            LOCAL_STACK_SIZE.with(|val| val.set(STACK_SIZE));
                                            Worker::work_loop(stealer, channel.clone(), wait, wait_mutex.clone(), deleting)
                                        })
                                 .expect("Error creating worker thread"));
        worker
    }

    fn work_loop<Message>(stealer: chase_lev::Stealer<Work<Message>>, channel: IoChannel<Message>, wait: Arc<SCondvar>, wait_mutex: Arc<SMutex<()>>, deleting: Arc<AtomicBool>)
    where
        Message: Send + Sync + Clone + 'static,
    {
        loop {
            {
                let lock = wait_mutex.lock().expect("Poisoned work_loop mutex");
                if deleting.load(AtomicOrdering::Acquire) {
                    return;
                }
                let _ = wait.wait(lock);
            }

            while !deleting.load(AtomicOrdering::Acquire) {
                match stealer.steal() {
                    chase_lev::Steal::Data(work) => Worker::do_work(work, channel.clone()),
                    _ => break,
                }
            }
        }
    }

    fn do_work<Message>(work: Work<Message>, channel: IoChannel<Message>)
    where
        Message: Send + Sync + Clone + 'static,
    {
        match work.work_type {
            WorkType::Readable => {
                work.handler.stream_readable(&IoContext::new(channel, work.handler_id), work.token);
            }
            WorkType::Writable => {
                work.handler.stream_writable(&IoContext::new(channel, work.handler_id), work.token);
            }
            WorkType::Hup => {
                work.handler.stream_hup(&IoContext::new(channel, work.handler_id), work.token);
            }
            WorkType::Timeout => {
                work.handler.timeout(&IoContext::new(channel, work.handler_id), work.token);
            }
            WorkType::Message(message) => {
                work.handler.message(&IoContext::new(channel, work.handler_id), &message);
            }
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        trace!(target: "shutdown", "[IoWorker] Closing...");
        let _ = self.wait_mutex.lock().expect("Poisoned work_loop mutex");
        self.deleting.store(true, AtomicOrdering::Release);
        self.wait.notify_all();
        if let Some(thread) = self.thread.take() {
            thread.join().ok();
        }
        trace!(target: "shutdown", "[IoWorker] Closed");
    }
}
