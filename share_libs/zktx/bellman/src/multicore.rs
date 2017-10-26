use crossbeam::{self, Scope, ScopedJoinHandle};
use num_cpus;

pub enum MaybeJoinHandle<T> {
    MultiThreaded(ScopedJoinHandle<T>),
    SingleThreaded(T)
}

impl<T> MaybeJoinHandle<T> {
    pub fn join(self) -> T {
        match self {
            MaybeJoinHandle::MultiThreaded(scope) => scope.join(),
            MaybeJoinHandle::SingleThreaded(t) => t
        }
    }
}

#[derive(Clone, Copy)]
pub enum MaybeScope<'a, 'b: 'a> {
    MultiThreaded(&'a Scope<'b>),
    SingleThreaded
}

impl<'a, 'b> MaybeScope<'a, 'b> {
    pub fn spawn<F, T>(&self, f: F) -> MaybeJoinHandle<T>
        where F: FnOnce() -> T + Send + 'b, T: Send + 'b
    {
        match self {
            &MaybeScope::MultiThreaded(scope) => MaybeJoinHandle::MultiThreaded(scope.spawn(f)),
            &MaybeScope::SingleThreaded => MaybeJoinHandle::SingleThreaded(f())
        }
    }
}

pub fn scope<'a, F, R>(
    elements: usize,
    f: F
) -> R where F: for<'b> FnOnce(MaybeScope<'b, 'a>, usize) -> R
{
    let num_cpus = num_cpus::get();

    if elements <= num_cpus {
        if elements == 0 {
            f(MaybeScope::SingleThreaded, 1)
        } else {
            f(MaybeScope::SingleThreaded, elements)
        }
    } else {
        crossbeam::scope(|scope| {
            f(MaybeScope::MultiThreaded(scope), elements / num_cpus)
        })
    }
}
