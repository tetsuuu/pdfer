mod config;
mod infrastructure;

use crate::config::CONFIG;
use axum::{
	routing::{get, post},
	Router,
};
use infrastructure::s3::S3Client;
use std::net::SocketAddr;
use std::path::Path;
use tracing;
use tracing_subscriber;

#[tokio::main]
async fn main() {
	// initialize tracing
	tracing_subscriber::fmt::init();

	// Initialize AWS Credentials
	let client = S3Client::get_aws_client(&CONFIG.region)?;
	let bucket_name = &CONFIG.bucket_name;
	let path = Path::new("/pdf");

	let app = Router::new()
		// Endpoint for Health check
		.route("/health", get(|| async { "ok" }))
		// list files
		.route("/get-files", post(S3Client::list_keys(&client, bucket_name)))
		// upload pdf
		.route("/upload", post(S3Client::upload_file(&client, bucket_name, path)));

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	tracing::debug!("listening on {}", addr);
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
