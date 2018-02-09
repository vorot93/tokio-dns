extern crate futures;

use std::net::{IpAddr, ToSocketAddrs};
use std::str;

use futures_pool::{Pool, Sender as PoolSender};
use tokio_io::IoFuture;
use std::sync::Arc;
use boxed;

/// The Resolver trait represents an object capable of
/// resolving host names into IP addresses.
pub trait Resolver {
    /// Given a host name, this function returns a Future which
    /// will eventually resolve into a list of IP addresses.
    fn resolve(&self, host: &str) -> IoFuture<Vec<IpAddr>>;
}

/// A resolver based on a thread pool.
///
/// This resolver uses the `ToSocketAddrs` trait inside
/// a thread to provide non-blocking address resolving.
#[derive(Clone)]
pub struct CpuPoolResolver {
    sender: Arc<PoolSender>,
    pool: Arc<Pool>,
}

impl CpuPoolResolver {
    /// Create a new CpuPoolResolver with the given number of threads.
    pub fn new(num_threads: usize) -> Self {
        let (sender, pool) = Pool::builder().pool_size(num_threads).build();
        let sender = Arc::new(sender);
        let pool = Arc::new(pool);
        Self {
            sender,
            pool,
        }
    }
}

impl Resolver for CpuPoolResolver {
    fn resolve(&self, host: &str) -> IoFuture<Vec<IpAddr>> {
        let host = format!("{}:0", host);
        boxed(futures::sync::oneshot::spawn(
            futures::future::lazy(move || match host[..].to_socket_addrs() {
                Ok(it) => Ok(it.map(|s| s.ip()).collect()),
                Err(e) => Err(e),
            }),
            &*self.sender,
        ))
    }
}
