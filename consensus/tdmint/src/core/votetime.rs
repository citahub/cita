// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use core::tendermint::Step;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

const THREAD_POOL_NUM: usize = 10;

#[derive(Debug, Default, Clone)]
pub struct TimeoutInfo {
    pub timeval: Duration,
    pub height: usize,
    pub round: usize,
    pub step: Step,
}
//unsafe impl ::std::marker::Sync for TimeoutInfo {}

pub struct WaitTimer {
    timer_seter: Receiver<TimeoutInfo>,
    timer_notify: Sender<TimeoutInfo>,
    thpool: ThreadPool,
}

//unsafe impl ::std::marker::Sync for WaitTimer {}

impl WaitTimer {
    pub fn new(ts: Sender<TimeoutInfo>, rs: Receiver<TimeoutInfo>) -> WaitTimer {
        let pool = ThreadPool::new(THREAD_POOL_NUM);
        WaitTimer {
            timer_notify: ts,
            timer_seter: rs,
            thpool: pool,
        }
    }

    pub fn start(&self) {
        let innersetter = &self.timer_seter;
        let zero_time = ::std::time::Duration::new(0, 0);
        loop {
            select! {
                settime = innersetter.recv() =>  {
                let oksettime = settime.unwrap();
                let notify = self.timer_notify.clone();

                if oksettime.timeval == zero_time {
                    notify.send(oksettime).unwrap();
                } else {
                    self.thpool.execute(move || {
                        trace!(" ************ {:?}",oksettime);
                        thread::sleep(oksettime.timeval);
                        notify.send(oksettime).unwrap();
                     });
                }

                }
            }
        }
    }
}
