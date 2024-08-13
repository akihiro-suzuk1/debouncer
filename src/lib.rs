use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;
use std::sync::Arc;

/// A debouncer that will call the last function after a delay.
/// 
/// # Examples
/// ```rust
/// use tokio::time::Duration;
/// use debouncer::Debouncer;
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
/// #[tokio::main]
/// async fn main() {
///     let debouncer = Debouncer::new(Duration::from_secs(1));
///     let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
/// 
///     let counter_clone: Arc<Mutex<i32>> = Arc::clone(&counter);
///     debouncer.call(move || {
///         async move {
///             let mut num = counter_clone.lock().await;
///             *num += 1;
///         }
///     }).await;
/// }
/// ```
pub struct Debouncer {
    delay: Duration,
    task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl Debouncer {
    /// Create a new debouncer with a delay.
    pub fn new(delay: Duration) -> Self {
        Debouncer {
            delay,
            task: Arc::new(Mutex::new(None)),
        }
    }

    /// Call the function after the delay.
    pub async fn call<F, Fut, R>(&self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = R> + Send + 'static,
    {
        let mut task = self.task.lock().await;

        // Cancel existing timer if it exists
        if let Some(handle) = task.take() {
            handle.abort();
        }

        // Set a new timer
        let delay = self.delay;
        *task = Some(tokio::spawn(async move {
            sleep(delay).await;
            f().await;
        }));
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::time::Duration;
    use crate::Debouncer;

    #[tokio::test]
    async fn should_call_last_only_test() {
        let debouncer = Debouncer::new(Duration::from_secs(1));
        let counter = Arc::new(Mutex::new(0));
    
        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            debouncer.call(move || {
                async move {
                    let mut num = counter_clone.lock().await;
                    *num += 1;
                }
            }).await;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    
        tokio::time::sleep(Duration::from_secs(2)).await;
    
        let final_value = *counter.lock().await;
        assert_eq!(final_value, 1, "The final call should be executed once.");
    }

    #[tokio::test]
    async fn should_call_all_if_duration_is_short_test() {
        let debouncer = Debouncer::new(Duration::from_millis(100));
        let counter = Arc::new(Mutex::new(0));
    
        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            debouncer.call(move || {
                async move {
                    let mut num = counter_clone.lock().await;
                    *num += 1;
                }
            }).await;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    
        tokio::time::sleep(Duration::from_secs(2)).await;
    
        let final_value = *counter.lock().await;
        assert_eq!(final_value, 5, "The final call should be executed every time.");
    }
}