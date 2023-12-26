use std::pin::Pin;

use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub(crate) trait AsyncMap<T, U, F>
    where
        F: FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>> + Send,
{
    type Output;
    async fn async_map(self, map: F) -> Self::Output;
}

#[async_trait]
impl<T, U, F> AsyncMap<T, U, F> for Option<T>
    where
        T: Send,
        U: Send,
        F: 'static + FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>> + Send,
{
    type Output = Option<U>;
    async fn async_map(self, map: F) -> Self::Output {
        match self {
            Some(t) => {
                let u = map(t).await;
                Some(u)
            }
            None => None,
        }
    }
}
