/* examples/axum.rs */

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

	println!("Server has been shut down gracefully.")
}
