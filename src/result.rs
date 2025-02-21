impl<T, E> crate::ResultExt for Result<T, E> {
    type T = T;
    type E = E;

    fn map_err_report(self) -> Result<Self::T, crate::report::Report<Self::E>>
    where
        Self::E: std::error::Error,
    {
        self.map_err(Into::<crate::report::Report<_>>::into)
    }

    fn expect_with<M, F: FnOnce() -> M>(self, f: F) -> Self::T
    where
        Self::E: std::fmt::Debug,
        M: AsRef<str>,
    {
        if let Ok(t) = self {
            t
        } else {
            let msg = f();

            #[allow(clippy::expect_fun_call)]
            self.expect(msg.as_ref())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ResultExt;
    use derive_more::Display;
    use derive_more::Error;

    // A mock error for testing purposes
    #[derive(Debug, Error, Display)]
    #[display("A mock error occurred")]
    struct MockError(MockSubError);

    // A mock error for testing purposes
    #[derive(Debug, Error, Display)]
    #[display("A mock sub error occurred")]
    struct MockSubError;

    // Sanity test for how this works with the std lib expect method that just uses the Debug impl to display the error
    #[test]
    #[should_panic(expected = "Custom panic: MockError(MockSubError)")]
    fn expect_err() {
        let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
        #[allow(clippy::unnecessary_literal_unwrap)]
        err_val.expect("Custom panic");
    }

    /// Tests the [ResultExt::map_err_report] method.
    mod map_err_report {
        use super::*;

        #[test]
        fn ok_unwrap() {
            let ok_val: Result<i32, MockError> = Ok(42);
            assert_eq!(ok_val.map_err_report().unwrap(), 42);
        }

        #[test]
        fn ok_expect() {
            let ok_val: Result<i32, MockError> = Ok(42);
            assert_eq!(ok_val.map_err_report().expect("Shouldn't see this"), 42);
        }

        #[test]
        #[should_panic(
            expected = "called `Result::unwrap()` on an `Err` value: A mock error occurred\n\nCaused by:\n      A mock sub error occurred"
        )]
        fn err_unwrap() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            err_val.map_err_report().unwrap();
        }

        #[test]
        #[should_panic(expected = "Custom report: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
        fn err_expect() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            err_val.map_err_report().expect("Custom report");
        }
    }

    /// Tests the [ResultExt::expect_with] method.
    mod expect_with {
        use super::*;

        #[test]
        fn ok() {
            let ok_val: Result<i32, MockError> = Ok(42);
            assert_eq!(ok_val.expect_with(|| "Shouldn't see this"), 42);
        }

        #[test]
        #[should_panic(expected = "Custom panic: MockError(MockSubError)")]
        fn err() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            err_val.expect_with(|| "Custom panic");
        }

        #[test]
        #[should_panic(expected = "Error in module A: MockError(MockSubError)")]
        fn err_format() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            let this_module = "module A";
            err_val.expect_with(|| format!("Error in {}", this_module));
        }

        #[test]
        #[should_panic(expected = "Custom panic: MockError(MockSubError)")]
        fn err_borrowed_string() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            let err_msg = String::from("Custom panic");
            err_val.expect_with(|| &err_msg);
        }
    }
}
