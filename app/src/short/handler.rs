use std::sync::Arc;

use actix_web::{web, Result};

use crate::short::model::CreateShort;
use crate::state::AppState;

use super::service::get_short_path;

pub fn config_shorten(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/shorten").route(web::put().to(put_url)));
}

async fn put_url(
  body: web::Json<CreateShort>,
  state: web::Data<Arc<AppState>>,
) -> Result<String> {
  let long_url = get_short_path(&state.pool, &state.opts.valid_scheme, body.url.as_str()).await?;

  Ok(long_url)
}
