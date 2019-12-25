# Averaged Collection

A Vec-based collection that maintains the average value of its elements. This is similar to the examples in [chapter 17 of the rust book](https://doc.rust-lang.org/book/ch17-01-what-is-oo.html).

# Example

```rust
use averaged_collection::AveragedCollection;

let mut avg_collection = AveragedCollection::new();
avg_collection.add(1);
avg_collection.add(2);

assert_eq!(avg_collection.average(), 1.5);
```

```rust
use averaged_collection::AveragedCollection;
use assert_approx_eq::assert_approx_eq;

let mut avg_collection = AveragedCollection::new();
avg_collection.add(1.2);
avg_collection.add(1.4);
avg_collection.add(1.6);

assert_approx_eq!(avg_collection.average(), 1.4);
```

# Installation

Add the following to your project's `Cargo.toml`

```toml
[dependencies]
averaged_collection = "*"
```
