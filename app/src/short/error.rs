use actix_web::http::uri::InvalidUri;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use actix_web::ResponseError;
use bb8::RunError;
use bb8_redis::redis::RedisError;

#[derive(Debug)]
pub enum ShortenErr {
  TimedOut,
  RedisError(RedisError),

  InvalidUri(InvalidUri),
  InvalidScheme,
  MissingScheme,
}

impl Display for ShortenErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl From<RunError<RedisError>> for ShortenErr {
  fn from(err: RunError<RedisError>) -> Self {
    match err {
      RunError::User(err) => ShortenErr::RedisError(err),
      RunError::TimedOut => ShortenErr::TimedOut,
    }
  }
}

impl From<RedisError> for ShortenErr {
  fn from(err: RedisError) -> Self {
    ShortenErr::RedisError(err)
  }
}

impl From<InvalidUri> for ShortenErr {
  fn from(err: InvalidUri) -> Self {
    ShortenErr::InvalidUri(err)
  }
}


impl Error for ShortenErr {}

impl ResponseError for ShortenErr {}
