# Shutdown Now

Gracefully handle shutdown and termination signals with zero hassle.

This Rust crate provides a simple way to wait for shutdown signals (like Ctrl+C or SIGTERM) in asynchronous applications, making it easy to implement graceful shutdowns in servers or long-running tasks.

## Features

- Supports Ctrl+C (SIGINT) on all platforms.
- Supports SIGTERM on Unix-like systems.
- Non-blocking future for flexible integration.
- Built on Tokio for asynchronous signal handling.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shutdown-now = "1"
```

Note: This crate depends on `tokio` with the `full` feature enabled.

## Usage

The crate exposes two main functions:

- `graceful()`: An async function that waits for a shutdown signal and prints a message when received.
- `graceful_future()`: Returns a future that completes when a shutdown signal is received, allowing non-blocking awaits.

### Basic Example

```rust
use shutdown_now::graceful;

#[tokio::main]
async fn main() {
    println!("App started, waiting for shutdown signal...");
    graceful().await;
    println!("Exiting gracefully...");
}
```

This will run until Ctrl+C is pressed or SIGTERM is sent, then print a shutdown message and exit.

### Integration with Axum

For web servers like Axum, use it with `with_graceful_shutdown`:

```rust
use axum::{Router, routing::get};
use shutdown_now::graceful;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world" }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server started on {}", addr);

    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(graceful())
        .await
    {
        eprintln!("Server error: {}", err);
    }

    println!("Server has been shut down gracefully.");
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

Check out the source code on [GitHub](https://github.com/canmi21/shutdown-now).