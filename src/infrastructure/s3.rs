#![allow(unused)]

use anyhow::{anyhow, bail, Context, Result};
use aws_sdk_s3::output::ListObjectsV2Output;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{config, Client, Credentials, Region};
use axum::async_trait;
use mime_guess;
use std::path::Path;

#[async_trait]
pub trait S3Client {
	fn get_aws_client(region: &str) -> Result<T>;
	async fn list_keys(client: &Client, bucket_name: &str) -> Result<()>;
	async fn upload_file(client: &Client, bucket_name: &str, path: Path) -> Result<()>;
}

pub struct AwsConfig {
	refion: String,
	bucket_name: String,
}

impl S3Client for AwsConfig {
	fn get_aws_client(region: &str) -> Result<Client> {
		let region = Region::new(region.to_string());
		let conf_builder = config::Builder::new().region(region);
		let conf = conf_builder.build();
		let client = Client::from_conf(conf);

		Ok(client)
	}

	async fn list_keys(client: &Client, bucket_name: &str) -> Result<Vec<String>> {
		let req = client.list_objects_v2().prefix("").bucket(bucket_name);

		let res = req.send().await?;

		let keys = res.contents().unwrap_or_default();
		let keys = keys.iter().filter_map(|o| o.key.as_ref()).map(|s| s.to_string()).collect::<Vec<_>>();

		Ok(keys)
	}

	async fn upload_file(client: &Client, bucket_name: &str, path: &Path) -> Result<()> {
		if !path.exists() {
			bail!("Path {} does not exists", path.display());
		}
		let key = path.to_str().ok_or_else(|| anyhow!("Invalid path {path:?}"))?;

		let body = ByteStream::from_path(&path).await?;
		let content_type = mime_guess::from_path(&path).first_or_octet_stream().to_string();

		let req = client.put_object().bucket(bucket_name).key(key).body(body).content_type(content_type);

		req.send().await?;

		Ok(())
	}
}
