use actix::{Actor, Addr};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::env;

mod game;
mod player;

async fn open_socket(
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
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9450".to_string());

    let game = game::Game::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game.clone()))
            .route("/", web::get().to(open_socket))
    })
    .workers(2)
    .bind(addr)?
    .run()
    .await
}
