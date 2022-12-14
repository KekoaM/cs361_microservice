use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_lab::web as web_lab;
use rand::Rng;
use serde::{Deserialize, Serialize};

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
    stamina: i32,
}

/// Generates a random name from a list, if the list is exhausted, attempts to pull updates for the
/// list from a 3rd party source TODO finish description
fn generate_name() -> String {
    // let mut unused_names: Vec<String> = Vec::new();
    let unused_names = [
        "Peasant Persivell",
        "Peasant Heriot",
        "Admiral Maukolum",
        "Grand Duchess Rechemay",
        "Landgrave Huon",
        "Bishop Conon",
        "Admiral Beavis",
        "Cardinal Triston",
        "Emperor Rich",
        "Archbishop Miles",
        "Princess Gaillart",
        "Baronetess Ingramus",
        "Count Rozeman",
        "Chancellor Karolus",
        "Viscount Arnald",
    ];
    // let mut used_names: Vec<String> = Vec::new();

    // unused_names.push("Test Name 0".to_owned());
    // unused_names.push("Test Name 1".to_owned());
    // unused_names.push("Test Name 2".to_owned());

    let mut rng = rand::thread_rng();

    let choice = rng.gen_range(0..unused_names.len());

    unused_names
        .iter()
        .nth(choice)
        .expect("rng range should be exact length of unused names list")
        .to_string()
}

/// Generates a semi-random set of stats based on the given difficulty
fn generate_stats(difficulty: i32) -> StatTable {
    let mut rng = rand::thread_rng();
    let name = generate_name();
    let health = rng.gen_range(100..200 * difficulty) * difficulty;
    let damage = rng.gen_range(100..200) * difficulty;
    let stamina = rng.gen_range(100 * difficulty..150 * difficulty * difficulty);

    let table: StatTable = StatTable {
        difficulty,
        name,
        health,
        damage,
        stamina,
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
            .service(web_lab::Redirect::new(
                "/",
                "https://github.com/KekoaM/cs361_microservice",
            ))
            .service(gen)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
