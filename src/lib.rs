/* src/lib.rs */

use std::future::Future;
use tokio::signal;

#[cfg(feature = "fancy-log")]
use fancy_log::{LogLevel, log, set_log_level};

/// Waits for a shutdown signal (Ctrl+C or SIGTERM).
pub async fn graceful() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("Failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("Failed to install SIGTERM handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	#[cfg(feature = "fancy-log")]
	{
		use std::env;
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
		log(LogLevel::Info, "Signal received, shutdown now...");
	}

	#[cfg(not(feature = "fancy-log"))]
	println!(" Signal received, shutdown now...");
}

/// Returns a Future that completes when shutdown signal occurs.
pub fn graceful_future() -> impl Future<Output = ()> {
	async { graceful().await }
}
