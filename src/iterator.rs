pub mod map_into;
pub mod map_opt;
pub mod map_res;
pub mod map_res_err;

impl<I> crate::IteratorExt for I
where
    I: Iterator,
{
    fn map_into<U>(self) -> map_into::MapInto<Self, U>
    where
        Self: Sized,
        Self: Iterator,
        <Self as Iterator>::Item: Into<U>,
    {
        map_into::MapInto {
            iter: self,
            _marker: std::marker::PhantomData,
        }
    }

    fn map_opt<T, U, F>(self, f: F) -> map_opt::MapOpt<Self, F>
    where
        Self: Sized,
        Self: Iterator<Item = Option<T>>,
        F: FnMut(T) -> U,
    {
        map_opt::MapOpt { iter: self, f }
    }

    fn map_res<F, T, U, E>(self, f: F) -> map_res::MapRes<Self, F>
    where
        Self: Sized,
        Self: Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> U,
    {
        map_res::MapRes { iter: self, f }
    }

    fn map_res_err<F, T, U, E>(self, f: F) -> map_res_err::MapResErr<Self, F>
    where
        Self: Sized,
        Self: Iterator<Item = Result<T, E>>,
        F: FnMut(E) -> U,
    {
        map_res_err::MapResErr { iter: self, f }
    }

    fn join_as_strings(self, separator: &str) -> String
    where
        Self: Iterator,
        <Self as Iterator>::Item: ToString,
    {
        // TODO: Use intersperse when it becomes stable
        self.map(|x| x.to_string()).collect::<Vec<_>>().join(separator)
    }
}
