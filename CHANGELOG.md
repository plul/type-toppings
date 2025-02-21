# Changelog

## Unreleased

- Added: `OptionExt` trait.
- Added: `OptionExt::assert_none` - Asserts the option to be `None`, panicking otherwise.
- Added: `OptionExt::debug_assert_none` - Asserts the option to be `None`, panicking otherwise, but only as a debug assertion.
- Changed: Features `option`, `result`, and `iterator` are enabled by default.
- Changed: Removed dependency on `error_reporter`.

## v0.2.1

- Added: Crate metadata to guide docs.rs documentation generation.
- Added: Feature "full" which enables all features.

## v0.2.0

- Changed(breaking): No features are enabled by default. Opt in to each extension trait with e.g. `type-toppings = { version = "0.2.0", features = ["iterator"] }`.
- Added: `IteratorExt::join_as_strings` - Converts each element of the iterator to a string and joins them into a single string, separated by the specified separator.

## v0.1.1

- Changed: `ResultExt::expect_with()` and `ResultExt::expect_or_report_with()` take a closure that now returns `impl AsRef<str>` rather than `&str`. This means a `String` can be returned, which enables use of the `format` macro in the closure.

## v0.1.0

- Initial release
