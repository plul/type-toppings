//! This module is inspired from [std::error::Report].

use std::error::Error;
use std::fmt::Write;
use std::fmt::{self};

/// Test report.
pub(crate) struct Report<E> {
    pub(crate) error: E,
}

impl<E: Error> From<E> for Report<E> {
    fn from(error: E) -> Self {
        Report { error }
    }
}

impl<E: Error> fmt::Display for Report<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = &self.error;

        write!(f, "{error}")?;

        if let Some(cause) = error.source() {
            write!(f, "\n\nCaused by:")?;

            let multiple = cause.source().is_some();

            for (ind, error) in Source::new(cause).enumerate() {
                writeln!(f)?;
                let mut indented = Indented { inner: f };
                if multiple {
                    write!(indented, "{ind: >4}: {error}")?;
                } else {
                    write!(indented, "      {error}")?;
                }
            }
        }

        Ok(())
    }
}

impl<E: Error> fmt::Debug for Report<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Clone, Debug)]
struct Source<'a> {
    current: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Source<'a> {
    fn new(error: &'a (dyn Error + 'static)) -> Self {
        Self { current: Some(error) }
    }
}

impl<'a> Iterator for Source<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(Error::source);
        current
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.current.is_some() { (1, None) } else { (0, Some(0)) }
    }
}

impl std::iter::FusedIterator for Source<'_> {}

struct Indented<'a, D> {
    inner: &'a mut D,
}

impl<T> Write for Indented<'_, T>
where
    T: Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if i > 0 {
                self.inner.write_char('\n')?;
                self.inner.write_str("      ")?;
            }

            self.inner.write_str(line)?;
        }

        Ok(())
    }
}
