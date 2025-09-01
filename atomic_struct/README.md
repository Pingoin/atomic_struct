# atomic_struct

`atomic_struct` is a Rust proc-macro that allows you to create **structs with atomic, asynchronous fields**. Each field is automatically wrapped in an `AtomicMember<T>` (`Arc<Mutex<T>`), providing **thread-safe async access** to individual fields.

All fields become private members of the stuct. Each field gets a getter and a setter method with the provided visibility.

---

## Features

- serde: activates serde compatibility fot the atomic-struct
  - fields are serilized without any mutex overhead

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
atomic_struct = { version="0.1.3" }
atomic_struct_core = { version="0.1.3" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Examples

``` rust
#[atomic_struct]
pub struct AppState {
    /// a public counter
    pub counter: i32,
    pub(crate) name: String,
} 
```

ist expanded to:

``` rust
pub struct AppState {
    /// a public counter
    counter: atomic_struct_core::AtomicMember<i32>,
    name: atomic_struct_core::AtomicMember<String>,
}

impl AppState {
    pub fn new(counter: i32, name: String) -> Self {
        Self {
            counter: atomic_struct_core::AtomicMember::new(counter),
            name: atomic_struct_core::AtomicMember::new(name),
        }
    }
    /// a public counter
    pub async fn get_counter(&self) -> i32 {
        self.counter.get().await
    }
    /// a public counter
    pub async fn set_counter(&self, new_val: i32) {
        self.counter.set(new_val).await
    }
    pub(crate) async fn get_name(&self) -> String {
        self.name.get().await
    }
    pub(crate) async fn set_name(&self, new_val: String) {
        self.name.set(new_val).await
    }
}
```

how to use:

For serde enshure the use of tokio MultiThread flavor.

```rust
use atomic_struct::atomic_struct;

#[atomic_struct]
pub struct AppState {
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
