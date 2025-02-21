impl<T> crate::OptionExt<T> for Option<T> {
    fn assert_none<M, F>(&self, f: F)
    where
        F: FnOnce(&T) -> M,
        M: AsRef<str>,
    {
        if let Some(t) = self {
            panic!("{}", f(t).as_ref());
        }
    }

    fn debug_assert_none<M, F>(&self, f: F)
    where
        F: FnOnce(&T) -> M,
        M: AsRef<str>,
    {
        if cfg!(debug_assertions) {
            self.assert_none(f);
        }
    }
}
