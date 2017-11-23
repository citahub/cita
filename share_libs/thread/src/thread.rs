use std::thread;
use std::sync::mpsc;
use std::sync::Arc;

pub enum TerminateMessage {
    Terminate,
}

pub struct Thread {
    thread: Option<thread::JoinHandle<()>>,
    sender_terminate: mpsc::Sender<TerminateMessage>,
}

type Fp = Box<Fn() -> () + Send + Sync>;

struct Closure {
    fp: Fp
}

impl Closure {
    fn call(&self) {
        (*self.fp)()
    }
}

impl Thread {
    pub fn spawnloop<F>(f: F) -> Thread where
        F: Fn() -> () + Send + Sync + 'static {

        let (sender, receiver) = mpsc::channel();
        let closure = Arc::new(Closure {fp: Box::new(f)});

        let thread = thread::spawn(move || {
            let f = closure.clone();
            loop {
                f.call();
                if let Ok(TerminateMessage::Terminate) = receiver.try_recv() {
                    break;
                }
            }
        });

        Thread {
            thread: Some(thread),
            sender_terminate: sender,
        }
    }

    pub fn join(&self) {}
}

impl Drop for Thread {
    fn drop(&mut self) {
        self.sender_terminate.send(TerminateMessage::Terminate).unwrap();
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
//目的：创建无线循环体，使其能够在主线程退出前，保证此处创建的线程能够完成非原子操作
//使用方法：
//步骤一：
//使用Thread::spawnloop创建包含无线混线的闭包执行体，
//注意此处不需要使用loop，该处的spawnloop本身就是创建无限循环体
//步骤二：
//从步骤一中获取到的线程结构体，执行join操作，thread.join(),
//其实该步骤中的join操作是空操作，只是需要拿到thread,使其在主线程退出时触发该线程以graceful的方式退出
//具体可以查看如下测试代码
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::sync::{Mutex, Arc};

    #[test]
    fn thread_join_loop() {
        println!("\n Begin to create thread defined by us");
        let check = Arc::new(Mutex::new(0));
        {
            let check_inner = check.clone();
            let thread = Thread::spawnloop(move || {
                {
                    let mut inner = check_inner.lock().unwrap();
                    *inner += 1;
                    println!("\n check changed to {}", *inner);
                }
                thread::sleep(Duration::new(1, 0));
            });
            thread::sleep(Duration::new(3, 0));
            thread.join();
        }
        assert_eq!(*check.lock().unwrap(), 3);
        println!("\n Thread is dropped automatically in graceful way now");
    }
}
