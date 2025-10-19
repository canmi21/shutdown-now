/* src/lib.rs */

use std::future::Future;
use tokio::signal;

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

	println!(" Signal received, shutdown now...");
}

/// Returns a Future that completes when shutdown signal occurs.
/// Same as [`graceful()`] but non-blocking (you can `.await` it anywhere).
pub fn graceful_future() -> impl Future<Output = ()> {
	async { graceful().await }
}
