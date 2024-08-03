use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;
use std::sync::Arc;

/// A simple debouncer for Rust, using Tokio for async timing.
///
/// # Examples
///
/// ```rust
/// use tokio::time::Duration;
/// use debouncer::Debouncer;
///
/// #[tokio::main]
/// async fn main() {
///     let debouncer = Debouncer::new(Duration::from_secs(1));
///
///     debouncer.call(|| println!("Function called!")).await;
///     debouncer.call(|| println!("Function called again!")).await;
///
///     for _ in 0..5 {
///         debouncer.call(|| println!("Called multiple times!")).await;
///         tokio::time::sleep(Duration::from_millis(200)).await;
///     }
///
///     tokio::time::sleep(Duration::from_secs(2)).await;
///     debouncer.call(|| println!("Final call!")).await;
/// }
/// ```
pub struct Debouncer {
    delay: Duration,
    task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl Debouncer {
    pub fn new(delay: Duration) -> Self {
        Debouncer {
            delay,
            task: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn call<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
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
            f();
        }));
    }
}

mod tests {
    use std::{sync::Arc, sync::Mutex, time::Duration};
    use crate::Debouncer;

    #[tokio::test]
    async fn should_call_last_only_test() {
        let debouncer = Debouncer::new(Duration::from_secs(1));
        let counter = Arc::new(Mutex::new(0));
    
        let counter_clone = Arc::clone(&counter);
        debouncer.call(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }).await;
    
        let counter_clone = Arc::clone(&counter);
        debouncer.call(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }).await;
    
        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            debouncer.call(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }).await;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    
        tokio::time::sleep(Duration::from_secs(2)).await;
    
        let final_value = *counter.lock().unwrap();
        assert_eq!(final_value, 1, "The final call should be executed once.");
    }

    #[tokio::test]
    async fn should_call_all_if_duration_is_short_test() {
        let debouncer = Debouncer::new(Duration::from_millis(100));
        let counter = Arc::new(Mutex::new(0));
    
        let counter_clone = Arc::clone(&counter);
        debouncer.call(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }).await;
    
        let counter_clone = Arc::clone(&counter);
        debouncer.call(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }).await;
    
        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            debouncer.call(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }).await;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    
        tokio::time::sleep(Duration::from_secs(2)).await;
    
        let final_value = *counter.lock().unwrap();
        assert_eq!(final_value, 5, "The final call should be executed every time.");
    }
}