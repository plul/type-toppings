/// An iterator adapter that converts the items of an inner iterator
/// producing `T` into `U` using the `Into` trait.
pub struct MapInto<I, U> {
    pub(crate) iter: I,
    pub(crate) _marker: std::marker::PhantomData<U>,
}

impl<I, T, U> Iterator for MapInto<I, U>
where
    I: Iterator<Item = T>,
    T: Into<U>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Into::into)
    }
}

impl<I, T, U> ExactSizeIterator for MapInto<I, U>
where
    I: ExactSizeIterator<Item = T>,
    T: Into<U>,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I, T, U> std::iter::FusedIterator for MapInto<I, U>
where
    I: Iterator<Item = T> + std::iter::FusedIterator,
    T: Into<U>,
{
}

#[cfg(test)]
mod tests {
    use crate::IteratorExt;

    #[test]
    fn test_map_into_basic() {
        let data = vec![1_u8, 3_u8];
        let result: Vec<u32> = data.into_iter().map_into().collect();
        assert_eq!(result, vec![1_u32, 3_u32]);
    }

    #[test]
    fn test_map_into_turbofish() {
        let data = vec![1_u8, 3_u8];
        let result: Vec<_> = data.into_iter().map_into::<u32>().collect();
        assert_eq!(result, vec![1_u32, 3_u32]);
    }

    #[test]
    fn test_map_into_len() {
        let data = vec![10_u8, 20, 30, 40];
        let iter = data.into_iter().map_into::<u32>();
        assert_eq!(iter.len(), 4);
    }
}
