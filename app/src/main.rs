use std::sync::Arc;

use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{http, web, App, HttpResponse, HttpServer};

use crate::opts::AppOpts;
use crate::short::handler::config_shorten;
use crate::short::middleware::Shorten;
use crate::state::AppState;

mod opts;
mod short;
mod state;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let opts = AppOpts::parse();
  let port = opts.port;
  let state = Arc::new(AppState::new(opts).await.unwrap());

  logfmt_logger::init();

  HttpServer::new(move || {
    App::new()
      .data(state.clone())
      .app_data(state.clone())
      .wrap(Logger::default())
      .wrap(Shorten)
      .wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
      .service(web::scope("/api").configure(config_shorten))
      .route("{_:.*}", web::get().to(redirect))
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}

async fn redirect(path: web::Path<String>, state: web::Data<Arc<AppState>>) -> HttpResponse {
  HttpResponse::PermanentRedirect()
    .header(
      http::header::LOCATION,
      format!("{}{}", &state.opts.web_url, path),
    )
    .finish()
}
