# atomic_struct

`atomic_struct` is a Rust proc-macro that allows you to create **structs with atomic, asynchronous fields**. Each field is automatically wrapped in an `AtomicMember<T>` (`Arc<Mutex<T>`), providing **thread-safe async access** to individual fields.

---

## Features

- `#[atomic_struct]` attribute macro for easy struct creation  
- Each field is wrapped in `AtomicMember<T>`  
- Automatically generated `new()` constructor  
- Async getter/setter for each field  
- Cloneable atomic fields  
- Optional: Serde field attributes are preserved (`#[serde(...)]`)  

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
atomic_struct = { path = "../atomic_struct" }  # or crates.io version when published
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Examples

```rust
use atomic_struct::atomic_struct;

#[atomic_struct]
struct AppState {
    counter: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let state = AppState::new(0, "init".to_string());

    // Async setters
    state.counter.set(42).await;
    state.name.set("hello".to_string()).await;

    // Async getters
    println!("Counter: {}", state.counter.get().await);
    println!("Name: {}", state.name.get().await);

    // Clone a single field
    let counter_clone = state.counter.clone();
    counter_clone.set(99).await;
    println!("Counter after clone: {}", state.counter.get().await);
}

```

## Advanced Usage

### Serde Support

If you add Serde field attributes like `#[serde(rename = "…")]` or `#[serde(skip)]` to the original fields, they will automatically be preserved.

```rust
#[atomic_struct]
struct AppState {
    #[serde(rename = "cnt")]
    counter: i32,
    #[serde(skip)]
    name: String,
}
```

### Global Singleton Usage

`atomic_struct` structs can be safely stored in `Arc` or `OnceCell`:

```rust
use once_cell::sync::OnceCell;
static GLOBAL_STATE: OnceCell<AppState> = OnceCell::new();
```

## Benefits

- Thread-safe and async by default
- Minimal boilerplate
- Cloneable fields for parallel tasks
- Automatic getters and setters
