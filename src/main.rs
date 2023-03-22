use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

const API_KEY: &str = "c8cd889ce6c02d80a1a4542a28287c67";

const PHRASES: [&str; 5] = [
    "Fall seven times, stand up eight.",
    "Turn your face toward the sun and the shadows fall behind you.",
    "A bird does not sing because it has an answer. It sings because it has a song.",
    "Those who wish to sing always find a song.",
    "It’s not enough to learn how to ride, you must also learn how to fall.",
];

#[get("/temperature/{lat}/{lon}")]
async fn greet(path: web::Path<(String, String)>) -> impl Responder {
    let (lat, lon) = path.into_inner();
    // call OpenWeatherMap API
    let response = reqwest::get(
        format!(
            "http://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&appid={}&exclude=current,minutely,hourly,alerts&units=metric",
            lat, lon, API_KEY
        )
        .as_str(),
    )
    .await;

    match response {
        Ok(response) => {
            let body = response.text().await;
            match body {
                Ok(body) => {
                    println!("{}", body);
                    let weather: OpenWeatherMap =
                        serde_json::from_str(body.as_str()).expect("Failed to parse weather data");
                    serde_json::to_string(&weather.daily)
                }
                Err(_) => serde_json::to_string(""),
            }
        }
        Err(_) => serde_json::to_string(""),
    }
}

#[get("/day-phrase")]
async fn day_phrase() -> impl Responder {
    // Get random phrase
    let phrase = PHRASES[rand::thread_rng().gen_range(0..PHRASES.len())];

    phrase
}

#[derive(Serialize, Deserialize)]
struct OpenWeatherMap {
    daily: Vec<Daily>,
}

#[derive(Serialize, Deserialize)]
struct Daily {
    temp: Temperature,
}

#[derive(Serialize, Deserialize)]
struct Temperature {
    day: f32,
    min: f32,
    max: f32,
    night: f32,
    eve: f32,
    morn: f32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(day_phrase))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
