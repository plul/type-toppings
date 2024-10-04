# Type Toppings

Opinionated collection of utility extensions for several of Rust's standard types, including:

- `Result`
- `Iterator`
- `futures::Steam`

## Documentation

<https://docs.rs/type-toppings/latest/type_toppings/>

## Example

```toml
# Cargo.toml

[dependencies]
type-toppings = { version = "0.2.0", features = ["iterator"] }
```

```rust
use type_toppings::IteratorExt;

// Map only the Some values in an iterator of Option<T>:
let data: Vec<_> = vec![Some(1), None, Some(3)]
   .into_iter()
   .map_opt(|x| x * 2)
   .collect();
assert_eq!(data, vec![Some(2), None, Some(6)]);
```
