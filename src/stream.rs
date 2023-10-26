impl<S> crate::StreamExt for S
where
    S: futures::stream::Stream,
{
    fn chain_ready<T>(self, item: T) -> futures::stream::Chain<Self, futures::stream::Once<std::future::Ready<T>>>
    where
        Self: Sized,
        Self: futures::Stream<Item = T>,
    {
        self.chain_future(std::future::ready(item))
    }

    fn chain_future<T, F>(self, fut: F) -> futures::stream::Chain<Self, futures::stream::Once<F>>
    where
        Self: Sized,
        Self: futures::Stream<Item = T>,
        F: core::future::Future<Output = T>,
    {
        futures::StreamExt::chain(self, futures::stream::once(fut))
    }
}

#[cfg(test)]
mod tests {
    use crate::StreamExt as _;

    #[test]
    fn test_chain_ready() {
        let initial_stream = futures::stream::iter(vec![1, 2, 3]);
        let chained_stream = initial_stream.chain_ready(4);

        let collected: Vec<_> = futures::executor::block_on_stream(chained_stream).collect();

        assert_eq!(collected, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_chain_future() {
        let initial_stream = futures::stream::iter(vec![1, 2, 3]);
        let chained_stream = initial_stream.chain_future(Box::pin(async { 4 }));

        let collected: Vec<_> = futures::executor::block_on_stream(chained_stream).collect();

        assert_eq!(collected, vec![1, 2, 3, 4]);
    }
}
