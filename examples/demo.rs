/* examples/demo.rs */

use shutdown_now::graceful;

#[tokio::main]
async fn main() {
	println!("App started, waiting for shutdown signal...");
	graceful().await;
	println!("Exiting gracefully...");
}
