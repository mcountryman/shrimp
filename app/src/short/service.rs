use bb8_redis::redis::AsyncCommands;
use bb8_redis::{redis, RedisPool};
use rand::{thread_rng, Rng};
use regex::Regex;

use super::{
  error::ShortenErr, validate::validate_scheme, URL_CHARACTERS, URL_IDEAL_LENGTH,
  URL_IDEAL_RETRIES, URL_KEY,
};

/// Gets short url path for supplied url.
/// 
/// # Arguments
/// * `pool` - The redis pull to insert shortened urls into.
/// * `valid_scheme` - The regex used for validating uri scheme.
/// * `long_url` - The url to shorten.
pub async fn get_short_path(
  pool: &RedisPool,
  valid_scheme: &Regex,
  long_url: &str,
) -> Result<String, ShortenErr> {
  let mut extra = 0usize;

  info!("add_url(\"{}\")", long_url);

  validate_scheme(&valid_scheme, long_url)?;

  loop {
    for _ in 0..URL_IDEAL_RETRIES {
      let short_url = get_short_url(extra);
      let short_url = short_url.as_str();
      let is_unique = try_push_url(&pool, short_url, long_url).await?;
      if is_unique {
        info!("is_unique = true");
        return Ok(short_url.to_owned());
      }

      info!("short_url = \"{}\"", short_url);
      info!("is_unique = false");
    }

    extra += 1;
  }
}

pub async fn to_long(pool: &RedisPool, short_url: &str) -> Result<String, ShortenErr> {
  info!("get_url(\"{}\")", short_url);

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
