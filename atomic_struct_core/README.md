# atomic_struct_core

`atomic_struct_core` provides the types used by the atomic_struct macros.

- `AtomicMember<T>` wraps the type `T` in `Arc<tokio::sync::Mutex<T>>` and adds methods to change the inner Value.

---

## Features

- serde: activates serde compatibility fot the AtomicMember
  - fields are serilized without any mutex overhead

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
atomic_struct_core = { version="0.1.3" }  # or crates.io version when published

```

## Examples

```rust
let atomic_int = AtomicMember::new(5);
assert_eq!(atomic_int.get().await, 5);
atomic_int.set(10).await;
assert_eq!(atomic_int.get().await, 10);
```
