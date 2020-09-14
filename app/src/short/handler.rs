use actix_web::{web, Result};

use crate::state::AppState;
use crate::short::model::CreateShort;

pub fn config_shorten(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/shorten")
      .route(web::put().to(put_url))
  );
}

async fn put_url(body: web::Json<CreateShort>, data: web::Data<AppState>) -> Result<String> {
  let shorten = &data.shorten;
  let long_url = shorten.add_url(body.url.as_str()).await?;

  Ok(long_url)
}