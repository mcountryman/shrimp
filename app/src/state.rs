use std::error::Error;

use bb8::Pool;
use bb8_redis::{RedisConnectionManager, RedisPool};

use crate::opts::AppOpts;

#[derive(Clone)]
pub struct AppState {
  pub opts: AppOpts,
  pub pool: RedisPool,
}

impl AppState {
  pub async fn new(opts: AppOpts) -> Result<Self, Box<dyn Error>> {
    let manager = RedisConnectionManager::new(opts.redis_url.as_str())?;
    let pool = Pool::builder().build(manager).await?;
    let pool = RedisPool::new(pool);

    Ok(Self { opts, pool })
  }
}
