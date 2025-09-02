use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Debug)]
/// A thread-safe, asynchronous wrapper around a value of type `T`.
pub struct AtomicMember<T>(Arc<Mutex<T>>);


impl<T> Clone for AtomicMember<T> {
    fn clone(&self) -> Self {
        AtomicMember(self.0.clone())
    }
}

impl<T> AtomicMember<T> {
    /// Creates a new `AtomicMember` wrapping the provided value.
    pub fn new(value: T) -> Self {
        AtomicMember(Arc::new(Mutex::new(value)))
    }
}

impl<T> Default for AtomicMember<T>
where T: Default{
    fn default() -> Self {
        AtomicMember::new(T::default())
    }
}

impl<T> AtomicMember<T> 
where T: Clone
{

    /// Asynchronously retrieves a clone of the inner value.
    pub async fn get(&self) -> T {
        let val = self.0.lock().await;
        val.clone()
    }

    /// Asynchronously sets the inner value to `new_val`.
    pub async fn set(&self, new_val: T) {
        let mut val = self.0.lock().await;
        *val = new_val;
    }

    pub fn get_sync(&self) -> T {
        tokio::task::block_in_place(|| self.0.blocking_lock()).clone()
    } 

    pub fn set_sync(&self, val:T){
        tokio::task::block_in_place(||{ 
            let mut guard=self.0.blocking_lock();
            *guard=val;
        });
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for AtomicMember<T>
where T: serde::Serialize + Clone
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        tokio::task::block_in_place(|| {
            let val = self.0.blocking_lock();
            val.serialize(serializer)
        })
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for AtomicMember<T>
where T: serde::Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = T::deserialize(deserializer)?;
        Ok(AtomicMember::new(val))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_atomic_member() {
        let atomic_int = AtomicMember::new(5);
        assert_eq!(atomic_int.get().await, 5);
        atomic_int.set(10).await;
        assert_eq!(atomic_int.get().await, 10);
    }

}
