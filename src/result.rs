impl<T, E> crate::ResultExt for Result<T, E> {
    type T = T;
    type E = E;

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

    fn expect_or_report(self, msg: &str) -> Self::T
    where
        Self::E: std::error::Error,
    {
        self.map_err(|err| error_reporter::Report::new(err).pretty(true)).expect(msg)
    }

    fn expect_or_report_with<M, F: FnOnce() -> M>(self, f: F) -> Self::T
    where
        Self::E: std::error::Error,
        M: AsRef<str>,
    {
        self.map_err(|err| error_reporter::Report::new(err).pretty(true)).expect_with(f)
    }

    fn unwrap_or_report(self) -> Self::T
    where
        Self::E: std::error::Error,
    {
        self.map_err(|err| error_reporter::Report::new(err).pretty(true))
            .expect("called `unwrap_or_report()` on an `Err` value")
    }
}

#[cfg(test)]
mod tests {
    use crate::ResultExt;

    // A mock error for testing purposes
    #[derive(Debug, derive_more::Error, derive_more::Display)]
    #[display("A mock error occurred")]
    struct MockError(MockSubError);

    // A mock error for testing purposes
    #[derive(Debug, derive_more::Error, derive_more::Display)]
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

    /// Tests the [ResultExt::expect_or_report] method.
    mod expect_or_report {
        use super::*;

        #[test]
        fn ok() {
            let ok_val: Result<i32, MockError> = Ok(42);
            assert_eq!(ok_val.expect_or_report("Shouldn't see this"), 42);
        }

        #[test]
        #[should_panic(expected = "Custom report: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
        fn err() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            err_val.expect_or_report("Custom report");
        }

        #[test]
        #[should_panic(expected = "Error in module A: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
        fn err_format() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            let this_module = "module A";
            err_val.expect_or_report(&format!("Error in {}", this_module));
        }
    }

    /// Tests the [ResultExt::expect_or_report_with] method.
    mod expect_or_report_with {
        use super::*;

        #[test]
        fn ok() {
            let ok_val: Result<i32, MockError> = Ok(42);
            assert_eq!(ok_val.expect_or_report_with(|| "Shouldn't see this"), 42);
        }

        #[test]
        #[should_panic(expected = "Dynamic report: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
        fn err() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            err_val.expect_or_report_with(|| "Dynamic report");
        }

        #[test]
        #[should_panic(expected = "Error in module A: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
        fn err_format() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            let this_module = "module A";
            err_val.expect_or_report_with(|| format!("Error in {}", this_module));
        }
    }

    /// Tests the [ResultExt::unwrap_or_report] method.
    mod unwrap_or_report {
        use super::*;

        #[test]
        fn ok() {
            let ok_val: Result<i32, MockError> = Ok(42);
            assert_eq!(ok_val.unwrap_or_report(), 42);
        }

        #[test]
        #[should_panic(
            expected = "called `unwrap_or_report()` on an `Err` value: A mock error occurred\n\nCaused by:\n      A mock sub error occurred"
        )]
        fn err() {
            let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
            err_val.unwrap_or_report();
        }
    }
}
