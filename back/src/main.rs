use actix::{Addr, Supervisor};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::env;

mod action;
mod game;
mod player;

#[get("/")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    game: web::Data<Addr<game::Game>>,
) -> Result<HttpResponse, Error> {
    // Create a player actor that will handle this socket connection
    let player = player::Player::new(game.get_ref().clone());

    ws::start(player, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9450".to_string());

    let game = Supervisor::start(|_| game::Game::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game.clone()))
            .service(index)
    })
    .bind(addr)?
    .run()
    .await
}
