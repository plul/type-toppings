/// An iterator adapter that applies a function to the items of an inner iterator
/// producing `Option<T>`, transforming the `Some` values.
#[derive(Debug)]
pub struct MapOpt<I, F> {
    pub(crate) iter: I,
    pub(crate) f: F,
}

impl<I, F, T, U> Iterator for MapOpt<I, F>
where
    I: Iterator<Item = Option<T>>,
    F: FnMut(T) -> U,
{
    type Item = Option<U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|opt| opt.map(&mut self.f))
    }
}

impl<I, F, T, U> ExactSizeIterator for MapOpt<I, F>
where
    I: ExactSizeIterator<Item = Option<T>>,
    F: FnMut(T) -> U,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I, F, T, U> std::iter::FusedIterator for MapOpt<I, F>
where
    I: Iterator<Item = Option<T>>,
    I: std::iter::FusedIterator,
    F: FnMut(T) -> U,
{
}

#[cfg(test)]
mod tests {
    use crate::IteratorExt;

    #[test]
    fn test_map_opt_basic() {
        let data = vec![Some(1), None, Some(3)];
        let result: Vec<_> = data.into_iter().map_opt(|x| x * 2).collect();
        assert_eq!(result, vec![Some(2), None, Some(6)]);
    }

    #[test]
    fn test_map_opt_empty() {
        let data: Vec<Option<i32>> = vec![];
        let result: Vec<_> = data.into_iter().map_opt(|x| x + 1).collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_map_opt_side_effects() {
        let mut count = 0;
        let data = vec![Some(1), None, Some(3)];
        let _result: Vec<_> = data
            .into_iter()
            .map_opt(|x| {
                count += 1;
                x * 2
            })
            .collect();
        assert_eq!(count, 2); // The closure should be invoked only for `Some` values.
    }

    #[test]
    fn test_map_opt_len() {
        let data = vec![Some(1), None, Some(3), None];
        let iter = data.into_iter().map_opt(|x| x * 2);
        assert_eq!(iter.len(), 4);
    }
}
