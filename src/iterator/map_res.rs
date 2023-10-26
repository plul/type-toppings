pub struct MapRes<I, F> {
    pub(crate) iter: I,
    pub(crate) f: F,
}

impl<I, F, T, U, E> Iterator for MapRes<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> U,
{
    type Item = Result<U, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|res| res.map(&mut self.f))
    }
}

impl<I, F, T, U, E> ExactSizeIterator for MapRes<I, F>
where
    I: ExactSizeIterator<Item = Result<T, E>>,
    F: FnMut(T) -> U,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I, F, T, U, E> std::iter::FusedIterator for MapRes<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    I: std::iter::FusedIterator,
    F: FnMut(T) -> U,
{
}
