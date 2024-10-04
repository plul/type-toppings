#[derive(Debug)]
pub struct MapResErr<I, F> {
    pub(crate) iter: I,
    pub(crate) f: F,
}

impl<I, F, T, E, U> Iterator for MapResErr<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(E) -> U,
{
    type Item = Result<T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|res| res.map_err(&mut self.f))
    }
}

impl<I, F, T, U, E> ExactSizeIterator for MapResErr<I, F>
where
    I: ExactSizeIterator<Item = Result<T, E>>,
    F: FnMut(E) -> U,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I, F, T, U, E> std::iter::FusedIterator for MapResErr<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    I: std::iter::FusedIterator,
    F: FnMut(E) -> U,
{
}
