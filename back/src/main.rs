use actix::{Actor, Addr, StreamHandler};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::env;

mod clientmanager;
mod gamemanager;
mod messages;

/// Entry point for our websocket route
async fn open_socket(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<gamemanager::GameManager>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        clientmanager::ClientManager::new(srv.get_ref().clone()),
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9450".to_string());

    let game_manager = gamemanager::GameManager::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_manager.clone()))
            .route("/", web::get().to(open_socket))
    })
    .workers(2)
    .bind(addr)?
    .run()
    .await
}
