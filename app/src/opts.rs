use dotenv::dotenv;
use regex::Regex;
use std::env;

#[derive(Clone)]
pub struct AppOpts {
  pub port: u16,
  pub web_url: String,
  pub app_url: String,
  pub redis_url: String,
  pub valid_scheme: Regex,
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

      app_url: env::var("NEXT_PUBLIC_APP_URL")
        .expect("Missing `NEXT_PUBLIC_APP_URL` environment variable.")
        .to_owned(),

      redis_url: env::var("REDIS_URL")
        .expect("Missing `REDIS_URL` environment variable.")
        .to_owned(),

      valid_scheme: Regex::new(
        &env::var("VALID_SCHEME").expect("Missing `VALID_SCHEMES` environment variable."),
      )
      .expect("Invalid regex in environment variable `VALID_SCHEMES`."),
    }
  }
}
