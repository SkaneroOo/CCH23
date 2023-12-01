use actix_web::{get, web::ServiceConfig, HttpResponse, web};
use shuttle_actix_web::ShuttleActixWeb;

// day -1

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
async fn fake_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

// day 1

#[get("/1/{nums}*")]
async fn cube_the_bits(path: web::Path<String>) -> String {
    let args = path.into_inner().as_str().rsplit('/')
                           .map(|s| s.parse::<isize>().expect("failed to parse number"))
                           .collect::<Vec<isize>>();
    let mut xored = 0;
    for arg in args {
        xored ^= arg;
    }
    let cube = xored.pow(3);
    format!("{cube}")
}

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(fake_error);
        cfg.service(cube_the_bits);
    };

    Ok(config.into())
}
