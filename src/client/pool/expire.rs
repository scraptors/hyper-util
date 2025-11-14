//! todo

use std::task::{Context, Poll};
use std::sync::{Arc, Mutex, Weak};

use tower_service::Service;


/// todo
pub  struct Expire<S> {
    inner: Arc<Mutex<S>>,
}

/// todo
pub struct Handle<S> {
    inner: Weak<Mutex<S>>,
}

// ===== impl Expire =====

impl<S, Req> Service<Req> for Expire<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.lock().unwrap().poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        self.inner.lock().unwrap().call(req)
    }
}

// ===== impl Handle =====

// impl<S> Handle<S> {
//     pub async fn when<'a, F>(&self, fut: F) -> Option<impl std::ops::DerefMut<Target = S>>
//     where
//         F: std::future::Future<Output = ()>,
//     {
//         fut.await;
//         // Some(self.inner.upgrade()?.lock().unwrap())
//         // todo!()
//     }
// }
