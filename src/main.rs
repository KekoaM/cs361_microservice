use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct Params {
    difficulty: i32,
}

#[derive(Serialize)]
/// Struct for the generated stats for each quey
struct StatTable {
    difficulty: i32,
    name: String,
    health: i32,
    damage: i32,
}

/// Generates a semi-random set of stats based on the given difficulty
fn generate_stats(difficulty: i32) -> StatTable {
    let mut rng = rand::thread_rng();
    let name = "Test Name".to_string();
    let health = rng.gen_range(10..100) * difficulty;
    let damage = (5 * difficulty).max(150 - health) + rng.gen_range(1..5) * difficulty;

    let table: StatTable = StatTable {
        difficulty,
        name,
        health,
        damage,
    };

    table
}

#[get("/gen")]
async fn gen(params: web::Query<Params>) -> impl Responder {
    web::Json(generate_stats(params.difficulty))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(gen)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
