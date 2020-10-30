#[macro_use]
extern crate log;

use std::sync::Arc;

use actix_web::{
  http::header::LOCATION, middleware::Logger, web, App, HttpResponse, HttpServer,
};

use actix_cors::Cors;

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

  logfmt_logger::init();

  let state = Arc::new(AppState::new(opts.clone()).await.unwrap());
  let port = opts.port;
  let web_url = opts.web_url;

  HttpServer::new(move || {
    App::new()
      .data(state.clone())
      .app_data(state.clone())
      .wrap(Cors::permissive())
      .wrap(Logger::default())
      .wrap(Shorten)
      .service(
        web::scope("/api") //
          .configure(config_shorten),
      )
      .route("{_:.*}", web::get().to(redirect))
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}

async fn redirect(path: web::Path<String>, state: web::Data<Arc<AppState>>) -> HttpResponse {
  HttpResponse::PermanentRedirect()
    .header(LOCATION, format!("{}{}", &state.opts.web_url, path))
    .finish()
}
