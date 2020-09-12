use std::error::Error;

use crate::opts::AppOpts;
use crate::short::service::ShortenService;

#[derive(Clone)]
pub struct AppState {
  pub shorten: ShortenService,
}

impl AppState {
  pub async fn new(opts: &AppOpts) -> Result<Self, Box<dyn Error>> {
    Ok(
      Self {
        shorten: ShortenService::new(opts.database.as_str()).await?
      }
    )
  }
}
