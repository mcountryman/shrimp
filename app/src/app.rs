

use mime_guess::from_path;
use rust_embed::RustEmbed;
use actix_web::{HttpResponse, web};
use actix_web::body::Body;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "../app-web/out/"]
struct Asset;

#[cfg(debug_assertions)]
pub fn config_app(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("")
      .route("",web::get().to(index))
      .route("_next/{_:.*}", web::get().to(assets))
  );
}

fn index() -> HttpResponse {
  handle_embedded_file("index.html")
}

fn assets(path: web::Path<(String, )>) -> HttpResponse {
  let path = path.0;
  let path = path.0;

  handle_embedded_file(&*format!("_next/{}", path))
}

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
      };

      HttpResponse::Ok()
        .content_type(
          from_path(path)
            .first_or_octet_stream()
            .as_ref()
        )
        .body(body)
    }
    None => HttpResponse::NotFound()
      .body("404 Not Found"),
  }
}