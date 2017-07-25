use std::time::Duration;

pub fn unix_now() -> Duration {
    ::std::time::UNIX_EPOCH.elapsed().unwrap()
}

pub trait AsMillis {
    fn as_millis(&self) -> u64;
}

impl AsMillis for Duration {
    fn as_millis(&self) -> u64 {
        self.as_secs() * 1_000 + (self.subsec_nanos() / 1_000_000) as u64
    }
}