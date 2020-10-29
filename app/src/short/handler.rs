use std::sync::Arc;

use actix_web::{web, Result};

use crate::short::model::CreateShort;
use crate::state::AppState;

use super::service::add_url;

pub fn config_shorten(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/shorten").route(web::put().to(put_url)));
}

async fn put_url(
  body: web::Json<CreateShort>,
  state: web::Data<Arc<AppState>>,
) -> Result<String> {
  let long_url = add_url(&state.pool, body.url.as_str()).await?;

  Ok(long_url)
}
