//! # Type Toppings: Extensions for Standard Rust Types
//!
//! Opinionated collection of utility extensions for several of Rust's standard types, including:
//! - `Result`
//! - `Iterator`
//! - `futures::Steam`
//!
//! # Examples:
//!
//! ```
//! #[cfg(feature = "iterator")]
//! {
//!     use type_toppings::IteratorExt;
//!    
//!     // Map only the Some values in an iterator of Option<T>:
//!     let data: Vec<_> = vec![Some(1), None, Some(3)]
//!         .into_iter()
//!         .map_opt(|x| x * 2)
//!         .collect();
//!     assert_eq!(data, vec![Some(2), None, Some(6)]);
//! }
//! ```
//!
//! For more detailed examples, see the documentation for each trait and method.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "iterator")]
mod iterator;

#[cfg(feature = "option")]
mod option;

#[cfg(feature = "result")]
mod result;

#[cfg(feature = "stream")]
mod stream;

pub mod report;

/// [`std::option::Option`] extensions.
///
/// Methods for the `Option` type.
#[cfg(feature = "option")]
pub trait OptionExt<T> {
    /// Asserts the option to be [`None`], panicking otherwise.
    ///
    /// The closure `f` is only evaluated if the option is [`Some`].
    ///
    /// # Panics
    ///
    /// Panics if the value is [`Some`], with a panic message provided by
    /// the closure `f`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use type_toppings::OptionExt as _;
    /// let x: Option<u32> = Some(0);
    /// x.assert_none(|value| format!("option contained a value: {value}"));
    /// ```
    fn assert_none<M, F>(&self, f: F)
    where
        F: FnOnce(&T) -> M,
        M: AsRef<str>;

    /// Asserts the option to be [`None`], panicking otherwise.
    ///
    /// The closure `f` is only evaluated if the option is [`Some`].
    ///
    /// When debug assertions are disabled, this has no effect.
    ///
    /// # Panics
    ///
    /// Panics if the value is [`Some`], with a panic message provided by
    /// the closure `f`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use type_toppings::OptionExt as _;
    /// let x: Option<u32> = Some(0);
    /// x.debug_assert_none(|value| format!("option contained a value: {value}"));
    /// ```
    fn debug_assert_none<M, F>(&self, f: F)
    where
        F: FnOnce(&T) -> M,
        M: AsRef<str>;
}

/// [`std::result::Result`] extensions.
///
/// Methods for the `Result` type for more descriptive unwrapping and error handling patterns.
#[cfg(feature = "result")]
pub trait ResultExt {
    /// Success value
    type T;

    /// Error value
    type E;

    /// Maps the error to [Report](crate::report::Report), a type that includes
    /// the chain of source errors when displaying the error.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use type_toppings::ResultExt as _;
    /// use derive_more::{Display, Error};
    ///
    /// #[derive(Debug, Error, Display)]
    /// #[display("Outer error")]
    /// struct OuterError(InnerError);
    ///
    /// #[derive(Debug, Error, Display)]
    /// #[display("Inner error")]
    /// struct InnerError;
    ///
    /// let x: Result<u32, OuterError> = Err(OuterError(InnerError));
    /// x.map_err_report().expect("Failure detected");
    /// ```
    /// The above panics with
    /// ```text
    /// Failure detected: Outer error
    ///
    /// Caused by:
    ///       Inner error
    /// ```
    fn map_err_report(self) -> Result<Self::T, crate::report::Report<Self::E>>
    where
        Self::E: std::error::Error;

    /// Unwraps the result, yielding the content of an [`Ok`].
    ///
    /// The closure `f` is only evaluated if the result contains an error.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message provided by
    /// the closure `f`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use type_toppings::ResultExt as _;
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.expect_with(|| "Testing expect_with");
    /// ```
    #[track_caller]
    fn expect_with<M, F: FnOnce() -> M>(self, f: F) -> Self::T
    where
        Self::E: std::fmt::Debug,
        M: AsRef<str>;
}

/// [`futures::Stream`] extensions.
#[cfg(feature = "stream")]
pub trait StreamExt {
    /// Chains a single ready item to the end of the stream.
    ///
    /// This method appends a ready item to the stream, effectively increasing the length of the
    /// stream by one. The item will be yielded after all items from the original stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_toppings::StreamExt as _;
    /// let initial_stream = futures::stream::iter(vec![1, 2, 3]);
    /// let chained_stream = initial_stream.chain_ready(4);
    ///
    /// let collected: Vec<_> = futures::executor::block_on_stream(chained_stream).collect();
    /// assert_eq!(collected, vec![1, 2, 3, 4]);
    /// ```
    fn chain_ready<T>(self, item: T) -> futures::stream::Chain<Self, futures::stream::Once<std::future::Ready<T>>>
    where
        Self: Sized,
        Self: futures::Stream<Item = T>;

    /// Chains a single future to the end of the stream.
    ///
    /// This method appends a future to the stream. When polled, the future will be awaited, and
    /// its resulting item will be yielded after all items from the original stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_toppings::StreamExt as _;
    /// let initial_stream = futures::stream::iter(vec![1, 2, 3]);
    /// let chained_stream = initial_stream.chain_future(Box::pin(async { 4 }));
    ///
    /// let collected: Vec<_> = futures::executor::block_on_stream(chained_stream).collect();
    /// assert_eq!(collected, vec![1, 2, 3, 4]);
    /// ```
    fn chain_future<T, F>(self, fut: F) -> futures::stream::Chain<Self, futures::stream::Once<F>>
    where
        Self: Sized,
        Self: futures::Stream<Item = T>,
        F: core::future::Future<Output = T>;
}

/// [`std::iter::Iterator`] extensions.
#[cfg(feature = "iterator")]
pub trait IteratorExt {
    /// Transforms the items in the iterator using the `Into` trait to convert
    /// from `T` to `U`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_toppings::IteratorExt;
    /// let data: Vec<_> = vec![1_u8, 3_u8]
    ///     .into_iter()
    ///     .map_into::<i32>()
    ///     .collect();
    /// assert_eq!(data, vec![1_i32, 3_i32]);
    /// ```
    fn map_into<U>(self) -> iterator::map_into::MapInto<Self, U>
    where
        Self: Sized,
        Self: Iterator,
        <Self as Iterator>::Item: Into<U>;

    /// Transforms the `Some` values in iterators of `Option<T>` using the given function `f`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_toppings::IteratorExt;
    /// let data: Vec<_> = vec![Some(1), None, Some(3)].into_iter().map_opt(|x| x * 2).collect();
    /// assert_eq!(data, vec![Some(2), None, Some(6)]);
    /// ```
    fn map_opt<T, U, F>(self, f: F) -> iterator::map_opt::MapOpt<Self, F>
    where
        Self: Sized,
        Self: Iterator<Item = Option<T>>,
        F: FnMut(T) -> U;

    /// Transforms the `Ok` values in iterators of `Result<T, E>` using the given function `f`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_toppings::IteratorExt;
    /// let data = [Ok(1), Err("some error"), Ok(3)]
    ///     .into_iter()
    ///     .map_res(|x| x * 2)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(data, vec![Ok(2), Err("some error"), Ok(6)]);
    /// ```
    fn map_res<F, T, U, E>(self, f: F) -> iterator::map_res::MapRes<Self, F>
    where
        Self: Sized,
        Self: Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> U;

    /// Transforms the `Err` values in iterators of `Result<T, E>` using the given function `f`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_toppings::IteratorExt;
    /// let data = [Ok(1), Err("unexpected thing happened"), Ok(3)]
    ///     .into_iter()
    ///     .map_res_err(|err| format!("Oh no: {err}"))
    ///     .collect::<Vec<_>>();
    /// assert_eq!(data, vec![Ok(1), Err("Oh no: unexpected thing happened".to_string()), Ok(3)]);
    /// ```
    fn map_res_err<F, T, U, E>(self, f: F) -> iterator::map_res_err::MapResErr<Self, F>
    where
        Self: Sized,
        Self: Iterator<Item = Result<T, E>>,
        F: FnMut(E) -> U;

    /// Converts each element of the iterator to a string and joins them into a single string, separated by the specified separator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_toppings::IteratorExt as _;
    /// let numbers = vec![1, 2, 3];
    /// let sequence = numbers.into_iter().join_as_strings(", ");
    /// assert_eq!(sequence, "1, 2, 3");
    ///
    /// let words = vec!["hello", "world"];
    /// let sentence = words.into_iter().join_as_strings(" - ");
    /// assert_eq!(sentence, "hello - world");
    /// ```
    fn join_as_strings(self, separator: &str) -> String
    where
        Self: Iterator,
        <Self as Iterator>::Item: ToString;
}
