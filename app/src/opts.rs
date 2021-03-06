use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppOpts {
  pub port: u16,
  pub web_url: String,
  pub redis_url: String,
}

impl AppOpts {
  pub fn parse() -> Self {
    dotenv().ok();

    Self {
      port: env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse()
        .expect("Invalid `PORT` environment variable."),

      web_url: env::var("WEB_URL")
        .expect("Missing `WEB_URL` environment variable.")
        .to_owned(),

      redis_url: env::var("REDIS_URL")
        .expect("Missing `REDIS_URL` environment variable.")
        .to_owned(),
    }
  }
}
