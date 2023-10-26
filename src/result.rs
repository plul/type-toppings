impl<T, E> crate::ResultExt for Result<T, E> {
    type T = T;
    type E = E;

    fn expect_with<'a, F: FnOnce() -> &'a str>(self, f: F) -> Self::T
    where
        Self::E: std::fmt::Debug,
    {
        if let Ok(t) = self {
            t
        } else {
            let msg = f();
            self.expect(msg)
        }
    }

    fn expect_or_report(self, msg: &str) -> Self::T
    where
        Self::E: std::error::Error,
    {
        self.map_err(|err| error_reporter::Report::new(err).pretty(true)).expect(msg)
    }

    fn expect_or_report_with<'a, F: FnOnce() -> &'a str>(self, f: F) -> Self::T
    where
        Self::E: std::error::Error,
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
    #[derive(Debug, thiserror::Error)]
    #[error("A mock error occurred")]
    struct MockError(#[source] MockSubError);

    // A mock error for testing purposes
    #[derive(Debug, thiserror::Error)]
    #[error("A mock sub error occurred")]
    struct MockSubError;

    // Sanity test for how this works with the std lib expect method that just uses the Debug impl to display the error
    #[test]
    #[should_panic(expected = "Custom panic: MockError(MockSubError)")]
    fn expect_err() {
        let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
        err_val.expect("Custom panic");
    }

    #[test]
    fn expect_with_ok() {
        let ok_val: Result<i32, MockError> = Ok(42);
        assert_eq!(ok_val.expect_with(|| "Shouldn't see this"), 42);
    }

    #[test]
    #[should_panic(expected = "Custom panic: MockError(MockSubError)")]
    fn expect_with_err() {
        let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
        err_val.expect_with(|| "Custom panic");
    }

    #[test]
    #[should_panic(expected = "Custom panic: MockError(MockSubError)")]
    fn expect_with_err_borrowed_string() {
        let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
        let err_msg = String::from("Custom panic");
        err_val.expect_with(|| &err_msg);
    }

    #[test]
    fn expect_or_report_ok() {
        let ok_val: Result<i32, MockError> = Ok(42);
        assert_eq!(ok_val.expect_or_report("Shouldn't see this"), 42);
    }

    #[test]
    #[should_panic(expected = "Custom report: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
    fn expect_or_report_err() {
        let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
        err_val.expect_or_report("Custom report");
    }

    #[test]
    fn expect_or_report_with_ok() {
        let ok_val: Result<i32, MockError> = Ok(42);
        assert_eq!(ok_val.expect_or_report_with(|| "Shouldn't see this"), 42);
    }

    #[test]
    #[should_panic(expected = "Dynamic report: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
    fn expect_or_report_with_err() {
        let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
        err_val.expect_or_report_with(|| "Dynamic report");
    }

    #[test]
    fn unwrap_or_report_ok() {
        let ok_val: Result<i32, MockError> = Ok(42);
        assert_eq!(ok_val.unwrap_or_report(), 42);
    }

    #[test]
    #[should_panic(expected = "called `unwrap_or_report()` on an `Err` value: A mock error occurred\n\nCaused by:\n      A mock sub error occurred")]
    fn unwrap_or_report_err() {
        let err_val: Result<i32, MockError> = Err(MockError(MockSubError));
        err_val.unwrap_or_report();
    }
}
