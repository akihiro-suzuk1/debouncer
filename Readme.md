# Debouncer

A simple debouncer for Rust, using Tokio for async timing. This crate provides a mechanism to debounce function calls, ensuring that only the last call within a specified delay is executed.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
debouncer = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Usage
Here’s a basic example demonstrating how to use the Debouncer:

```rust
use tokio::time::Duration;
use debouncer::Debouncer;

#[tokio::main]
async fn main() {
    let debouncer = Debouncer::new(Duration::from_secs(1));
    let counter = Arc::new(Mutex::new(0));

    let counter_clone = Arc::clone(&counter);
    debouncer.call(move || {
        async move {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }
    }).await;
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

	1.	Fork the repository
	2.	Create your feature branch (git checkout -b feature/your-feature)
	3.	Commit your changes (git commit -am 'Add some feature')
	4.	Push to the branch (git push origin feature/your-feature)
	5.	Create a new Pull Request

## License

This project is licensed under the MIT or Apache-2.0 License. See the LICENSE file for details.