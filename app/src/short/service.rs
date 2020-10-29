use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use actix_web::ResponseError;
use bb8::RunError;
use bb8_redis::redis::{AsyncCommands, RedisError};
use bb8_redis::{redis, RedisPool};
use rand::{thread_rng, Rng};

pub const URL_KEY: &str = "urls";
pub const URL_CHARACTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const URL_IDEAL_LENGTH: usize = 5;
pub const URL_IDEAL_RETRIES: u32 = 3;

#[derive(Debug)]
pub enum ShortenErr {
  Redis(RedisError),
  RunError(RunError<RedisError>),
}

pub async fn add_url(pool: &RedisPool, long_url: &str) -> Result<String, ShortenErr> {
  let mut extra = 0usize;

  loop {
    for _ in 0..URL_IDEAL_RETRIES {
      let short_url = get_short_url(extra);
      let short_url = short_url.as_str();
      let is_unique = try_push_url(&pool, short_url, long_url).await?;
      if is_unique {
        return Ok(short_url.to_owned());
      }
    }

    extra += 1;
  }
}

pub async fn get_url(pool: &RedisPool, short_url: &str) -> Result<String, ShortenErr> {
  let mut conn = pool.get().await?;
  let conn = conn.as_mut().unwrap();
  let long_url: String = conn.hget(URL_KEY, short_url).await?;

  Ok(long_url)
}

fn get_short_url(extra: usize) -> String {
  let mut rng = thread_rng();

  (0..URL_IDEAL_LENGTH + extra)
    .map(|_| {
      let ch_id = rng.gen_range(0, URL_CHARACTERS.len());
      URL_CHARACTERS[ch_id] as char
    })
    .collect()
}

async fn try_push_url(
  pool: &RedisPool,
  short_url: &str,
  long_url: &str,
) -> Result<bool, ShortenErr> {
  let mut conn = pool.get().await?;
  let conn = conn.as_mut().unwrap();
  let code: i32 = redis::cmd("HSETNX")
    .arg(&[URL_KEY, short_url, long_url])
    .query_async(conn)
    .await?;

  Ok(code > 0)
}

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
