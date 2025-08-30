#[derive(Debug)]
pub struct AtomicMember<T>(std::sync::Arc<tokio::sync::Mutex<T>>);


impl<T> Clone for AtomicMember<T> {
    fn clone(&self) -> Self {
        AtomicMember(self.0.clone())
    }
}
impl<T> AtomicMember<T> {
    pub fn new(value: T) -> Self {
        AtomicMember(std::sync::Arc::new(tokio::sync::Mutex::new(value)))
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

    pub async fn get(&self) -> T {
        let val = self.0.lock().await;
        val.clone()
    }

    pub async fn set(&self, new_val: T) {
        let mut val = self.0.lock().await;
        *val = new_val;
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
        let val = self.0.blocking_lock();
        val.serialize(serializer)
    }
}