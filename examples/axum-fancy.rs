/* examples/axum-fancy.rs */

use axum::{Router, routing::get};
use fancy_log::{LogLevel, log, set_log_level};
use shutdown_now::graceful;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let level = env::var("LOG_LEVEL")
		.unwrap_or_else(|_| "info".to_string())
		.to_lowercase();
	let level = match level.as_str() {
		"debug" => LogLevel::Debug,
		"warn" => LogLevel::Warn,
		"error" => LogLevel::Error,
		_ => LogLevel::Info,
	};
	set_log_level(level);

	let app = Router::new().route("/", get(|| async { "Hello, world" }));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

	let listener = TcpListener::bind(addr).await.unwrap();
	log(LogLevel::Info, &format!("Server started on {}", addr));

	if let Err(err) = axum::serve(listener, app)
		.with_graceful_shutdown(graceful())
		.await
	{
		log(LogLevel::Error, &format!("Server error: {}", err));
	}

	log(LogLevel::Info, "Server has been shut down gracefully.");
}
