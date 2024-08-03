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

    debouncer.call(|| println!("Function called!")).await;
    debouncer.call(|| println!("Function called again!")).await;

    for _ in 0..5 {
        debouncer.call(|| println!("Called multiple times!")).await;
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    tokio::time::sleep(Duration::from_secs(2)).await;
    debouncer.call(|| println!("Final call!")).await;
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