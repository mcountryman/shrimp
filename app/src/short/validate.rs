use regex::Regex;
use actix_web::http::Uri;
use super::error::ShortenErr;

pub fn validate_scheme(regex: &Regex, url: &str) -> Result<(), ShortenErr> {
  let uri = url.parse::<Uri>()?;
  match uri.scheme() {
    Some(scheme) => if regex.is_match(scheme.as_str()) {
      Ok(())
    } else {
      Err(ShortenErr::InvalidScheme)
    },
    None => Err(ShortenErr::MissingScheme),
  }
}
