use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use actix_web::ResponseError;
use bb8::{Pool, RunError};
use bb8_redis::{redis, RedisConnectionManager, RedisPool};
use bb8_redis::redis::{AsyncCommands, RedisError};
use rand::{Rng, thread_rng};

pub const URL_KEY: &str = "urls";
pub const URL_CHARACTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const URL_IDEAL_LENGTH: usize = 5;
pub const URL_IDEAL_RETRIES: u32 = 3;

#[derive(Debug)]
pub enum ShortenErr {
  Redis(RedisError),
  RunError(RunError<RedisError>),
}

#[derive(Clone, Debug)]
pub struct ShortenService {
  pool: RedisPool,
}

impl ShortenService {
  pub async fn new(url: &str) -> Result<Self, ShortenErr> {
    let manager = RedisConnectionManager::new(url)?;
    let pool = Pool::builder()
      .build(manager)
      .await?;

    let pool = RedisPool::new(pool);

    Ok(Self { pool })
  }

  pub async fn add_url(&self, long_url: &str) -> Result<String, ShortenErr> {
    let mut extra = 0usize;

    loop {
      for _ in 0..URL_IDEAL_RETRIES {
        let short_url = self.get_short_url(extra);
        let short_url = short_url.as_str();
        let is_unique = self.try_push_url(short_url, long_url).await?;
        if is_unique {
          return Ok(short_url.to_owned());
        }
      }

      extra += 1;
    }
  }

  pub async fn get_url(&self, short_url: &str) -> Result<String, ShortenErr> {
    let mut conn = self.pool.get().await?;
    let conn = conn.as_mut().unwrap();
    let long_url: String = conn.hget(URL_KEY, short_url).await?;

    Ok(long_url)
  }

  fn get_short_url(&self, extra: usize) -> String {
    let mut rng = thread_rng();

    (0..URL_IDEAL_LENGTH + extra)
      .map(|_| {
        let ch_id = rng.gen_range(0, URL_CHARACTERS.len());
        URL_CHARACTERS[ch_id] as char
      })
      .collect()
  }

  async fn try_push_url(
    &self,
    short_url: &str,
    long_url: &str,
  ) -> Result<bool, ShortenErr> {
    let mut conn = self.pool.get().await?;
    let conn = conn.as_mut().unwrap();
    let code: i32 = redis::cmd("HSETNX")
      .arg(&[URL_KEY, short_url, long_url])
      .query_async(conn)
      .await?;

    Ok(code > 0)
  }
}

impl Unpin for ShortenService {}

impl Display for ShortenErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl From<RedisError> for ShortenErr {
  fn from(err: RedisError) -> Self {
    ShortenErr::Redis(err)
  }
}

impl From<RunError<RedisError>> for ShortenErr {
  fn from(err: RunError<RedisError>) -> Self {
    ShortenErr::RunError(err)
  }
}

impl Error for ShortenErr {}

impl ResponseError for ShortenErr {}
