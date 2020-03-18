#[cfg(feature = "runtime_asyncstd")]
extern crate async_std;
extern crate futures;
#[cfg(feature = "runtime_bastion")]
extern crate lightproc;
#[cfg(feature = "runtime_tokio")]
extern crate tokio;

mod join_handle;

use join_handle::JoinHandle;
use std::future::Future;

pub trait AgnostikExecutor {
    fn spawn<F, T>(future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;

    fn spawn_blocking<F, T>(future: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;

    fn block_on<F, T>(future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}
