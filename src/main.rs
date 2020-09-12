use actix_web::{App, HttpServer, web};
use clap::Clap;

use crate::opts::AppOpts;
use crate::short::handler::config_shorten;
use crate::short::middleware::Shorten;
use crate::state::AppState;
use actix_web::middleware::DefaultHeaders;

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
      .service(
        web::scope("/api")
          .configure(config_shorten)
      )
  })
    .bind(format!("0.0.0.0:{}", opts.port))?
    .run()
    .await
}
