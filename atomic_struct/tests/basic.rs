use atomic_struct::atomic_struct;

#[atomic_struct]
#[derive(Debug, Clone, Default)]
pub struct AppState {
    /// Ein Zähler
    pub counter: i32,
    /// Ein Name
    pub(crate) name: String,
} 

#[tokio::main]
async fn main() {
    let state = AppState::new(0, "init".to_string());

    state.set_counter(42).await;
    state.set_name("hallo".to_string()).await;

    dbg!("State: {:?}", &state);
    println!("Counter: {}", state.get_counter().await);
    println!("Name: {}", state.get_name().await);
}

#[tokio::test]
async fn test_atomic_struct() {
    let state = AppState::new(0, "init".to_string());   
    assert_eq!(state.get_counter().await, 0);
    assert_eq!(state.get_name().await, "init".to_string());
    state.set_counter(100).await;
    state.set_name("changed".to_string()).await;
    assert_eq!(state.get_counter().await, 100);
    assert_eq!(state.get_name().await, "changed".to_string());
}