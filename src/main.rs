use std::collections::HashMap;

use serde::Deserialize;

use actix_web::{get, post, web::ServiceConfig, HttpResponse, web};
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

// day 4

#[derive(Deserialize)]
struct Data1 {
    name: String,
    strength: isize,
}

#[post("/4/strength")]
async fn strength(data: web::Json<Vec<Data1>>) -> String {
    let data = data.into_inner();

    let mut strength = 0;
    for value in data {
        strength += value.strength;
    }
    
    format!("{strength}")
}

#[derive(Deserialize)]
struct Data2 {
    name: String,
    strength: isize,
    speed: f64,
    antler_width: isize,
    snow_magic_power: isize,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies: isize
}

#[post("/4/contest")]
async fn contest(data: web::Json<Vec<Data2>>) -> HttpResponse {

    let mut results = HashMap::new();

    let data = data.into_inner();

    let mut speed = 0.0;
    let mut width = 0;
    let mut magic = 0;
    let mut candies = 0;

    for raindeer in data {
        if raindeer.speed > speed {
            speed = raindeer.speed;
            results.insert(
                "fastest", 
                format!("Speeding past the finish line with a strength of {} is {}", raindeer.speed, raindeer.name)
            );
        }
        
        if raindeer.antler_width > width {
            width = raindeer.antler_width;
            results.insert(
                "tallest", 
                format!("{} is standing tall with his {} cm wide antlers", raindeer.name, raindeer.antler_width)
            );
        }
        
        if raindeer.candies > magic {
            magic = raindeer.candies;
            results.insert(
                "magician", 
                format!("{} could blast you away with a snow magic power of {}", raindeer.name, raindeer.snow_magic_power)
            );
        }
        
        if raindeer.snow_magic_power > candies {
            candies = raindeer.snow_magic_power;
            results.insert(
                "consumer", 
                format!("{} ate lots of candies, but also some {}", raindeer.name, raindeer.favorite_food)
            );
        }
    }
    
    HttpResponse::Ok().json(results)
}

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(fake_error);
        cfg.service(cube_the_bits);
        cfg.service(strength);
        cfg.service(contest);
    };

    Ok(config.into())
}
