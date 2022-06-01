use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(Config::from_env);

pub struct Config {
	pub env: String,
	pub database_url: String,
	pub bucket_name: String,
	pub region: String,
}

#[derive(Debug, PartialEq)]
pub enum Env {
	Local,
	Stg,
	Prod,
}

impl Config {
	pub fn from_env() -> Self {
		let env = std::env::var("ENV").unwrap_or_else(|_| "local".to_string()).as_str().into();

		let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://pdfer:pdfer@localhost:5432/pdfer".to_string());

		let bucket_name = std::env::var("BUCKET_NAME").unwrap_or_else(|_| "hogehoge.com".to_string());

		let region = std::env::var("DEFAULT_REGION").unwrap_or_else(|_| "ap-northeast-1".to_string());

		Config {
			env,
			database_url,
			bucket_name,
			region,
		}
	}
}

impl<'a, T: Into<&'a str>> From<T> for Env {
	fn from(s: T) -> Self {
		match s.into() {
			"stg" => Env::Stg,
			"prod" => Env::Prod,
			_ => Env::Local,
		}
	}
}

impl ToString for Env {
	fn to_string(&self) -> String {
		match self {
			Env::Local => "local".to_string(),
			Env::Stg => "stg".to_string(),
			Env::Prod => "prod".to_string(),
		}
	}
}
