use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::{cell::RefCell, sync::Arc};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, http, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;

use crate::short::service::{URL_CHARACTERS, URL_IDEAL_LENGTH};
use crate::state::AppState;

use super::service::get_url;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Shorten;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S> for Shorten
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = ShortenMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(ShortenMiddleware {
      service: Rc::new(RefCell::new(service)),
    })
  }
}

pub struct ShortenMiddleware<S> {
  service: Rc<RefCell<S>>,
}

impl<S> ShortenMiddleware<S> {
  pub fn get_short_url_candidate(&self, path: &str) -> Option<String> {
    let mut short_url = String::new();
    let mut eat_leading_slashes = false;
    let mut ate_leading_slashes = false;
    let mut ate_trailing_slashes = false;
    let mut eat_trailing_slashes = false;

    for ch in path.chars() {
      if ch == '\\' || ch == '/' {
        if !ate_leading_slashes {
          eat_leading_slashes = true;
          continue;
        }

        if !ate_trailing_slashes {
          eat_trailing_slashes = true;
          continue;
        }

        return None;
      }

      if eat_leading_slashes {
        ate_leading_slashes = true;
      }

      if eat_trailing_slashes {
        ate_trailing_slashes = true;
      }

      if ate_trailing_slashes {
        return None;
      }
      if URL_CHARACTERS.contains(&(ch as u8)) {
        short_url.push(ch);
        continue;
      }

      return None;
    }

    if short_url.len() < URL_IDEAL_LENGTH {
      None
    } else {
      Some(short_url)
    }
  }
}

type ShortenFuture<A, B> = Pin<Box<dyn Future<Output = Result<A, B>>>>;

impl<S: 'static, B> Service for ShortenMiddleware<S>
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = ShortenFuture<Self::Response, Self::Error>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&mut self, req: ServiceRequest) -> Self::Future {
    let short_url = self.get_short_url_candidate(req.path());
    if short_url.is_none() {
      return Box::pin(self.service.call(req));
    }

    Box::pin(async move {
      let state = req.app_data::<Arc<AppState>>().unwrap();
      let short_url = short_url.unwrap();
      let redirect = get_url(&state.pool, short_url.as_str()).await?;
      let redirect = HttpResponse::TemporaryRedirect()
        .header(http::header::LOCATION, redirect)
        .finish()
        .into_body();

      Ok(req.into_response(redirect))
    })
  }
}
