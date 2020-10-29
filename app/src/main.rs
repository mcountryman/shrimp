use std::borrow::Cow;

use actix_web::{App, HttpResponse, HttpServer, web, http};
use actix_web::middleware::DefaultHeaders;
use clap::Clap;

use crate::opts::AppOpts;
use crate::short::handler::config_shorten;
use crate::short::middleware::Shorten;
use crate::state::AppState;

mod app;
mod opts;
mod short;
mod state;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let opts: AppOpts = AppOpts::parse();
  let state = AppState::new(&opts).await.unwrap();

  HttpServer::new(move || {
    App::new()
      .data(state.clone())
      .app_data(state.clone())
      .wrap(Shorten)
      .wrap(DefaultHeaders::new()
        .header("Access-Control-Allow-Origin", "*")
      )
      .service(web::scope("/api").configure(config_shorten))
      .route("{_:.*}", web::get().to(redirect))
  })
    .bind(format!("0.0.0.0:{}", opts.port))?
    .run()
    .await
}

async fn redirect(path: web::Path<String>) -> HttpResponse {
  HttpResponse::PermanentRedirect()
    .header(http::header::LOCATION, format!("https://www.shr1.mp/{}", path))
    .finish()
}