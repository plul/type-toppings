# Changelog

## Unreleased

## v0.1.1

- `ResultExt::expect_with()` and `ResultExt::expect_or_report_with()` take a closure that now returns `impl AsRef<str>` rather than `&str`. This means a `String` can be returned, which enables use of the `format` macro in the closure.

## v0.1.0

- Initial release
