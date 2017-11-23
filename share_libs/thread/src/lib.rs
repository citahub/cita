//wrapper the stand thread to enable graceful shutdown
//具体的使用方法见thread.rs的test
pub mod thread;

pub use thread::*;
