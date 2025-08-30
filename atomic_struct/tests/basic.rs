use atomic_struct::atomic_struct;

#[atomic_struct]
struct AppState {
    counter: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let state = AppState::new(0, "init".to_string());

    state.set_counter(42).await;
    state.set_name("hallo".to_string()).await;

    println!("Counter: {}", state.get_counter().await);
    println!("Name: {}", state.get_name().await);
}
