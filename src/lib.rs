use lazy_static::lazy_static;
use std::{future::Future, sync::Arc};
use tokio::runtime::Runtime;

lazy_static! {
    static ref GLOBAL_TOKIO: Arc<Runtime> = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    );
}

/// spawn some lame task that requires tokio, use global tokio runtime
pub fn spawn<Fut>(fut: Fut)
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    GLOBAL_TOKIO.spawn(fut);
}
