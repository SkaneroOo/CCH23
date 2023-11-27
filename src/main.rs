use actix_web::{get, web::ServiceConfig, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
async fn fake_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(fake_error);
    };

    Ok(config.into())
}
