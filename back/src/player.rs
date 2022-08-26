use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

use crate::game;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_PONG_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Player {
    pub id: Option<u8>,
    /// The timestamp of the last received pong. If more than
    /// `CLIENT_PONG_TIMEOUT` elapsed since then, then the connection is assumed
    /// to be down.
    pub last_pong: Instant,
    pub game: Addr<game::Game>,
}

#[derive(Message)]
#[rtype("()")]
pub struct PlayerId(pub u8);

#[derive(Message)]
#[rtype("()")]
pub struct TurnEnded;

#[derive(Message)]
#[rtype("()")]
pub struct GameEnded;

#[derive(Serialize)]
#[serde(tag = "t", rename_all = "camelCase")]
enum OutSocketMessage {
    Id { id: u8 },
    TurnEnded,
    GameEnded,
}

#[derive(Deserialize)]
#[serde(tag = "t", rename_all = "camelCase")]
enum InSocketMessage {
    EndTurn,
}

impl Actor for Player {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.setup_heartbeat(ctx);

        // Register the player to the global game.
        self.game.do_send(game::Connect {
            player: ctx.address(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.game.do_send(game::Disconnect);
        Running::Stop
    }
}

impl Player {
    pub fn new(game: Addr<game::Game>) -> Player {
        Player {
            id: None,
            last_pong: Instant::now(),
            game,
        }
    }

    fn setup_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
            if actor.last_pong.elapsed() >= CLIENT_PONG_TIMEOUT {
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }

    fn handle_socket_message(&self, msg: InSocketMessage) {
        match msg {
            InSocketMessage::EndTurn => {
                // TODO: Don't unwrap
                self.game.do_send(game::EndTurn {
                    player_id: self.id.unwrap(),
                });
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Player {
    fn handle(
        &mut self,
        maybe_msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match maybe_msg {
            Ok(msg) => match msg {
                ws::Message::Text(text) => {
                    match serde_json::from_str::<InSocketMessage>(&text) {
                        Err(err) => {
                            log::error!("Ignoring malformed message '{:?}': {}", text, err);
                            return;
                        }
                        Ok(socket_msg) => self.handle_socket_message(socket_msg),
                    };
                }
                ws::Message::Ping(bytes) => {
                    ctx.pong(&bytes);
                }
                ws::Message::Pong(_bytes) => {
                    self.last_pong = Instant::now();
                }
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                _ => (),
            },
            Err(_) => ctx.stop(),
        }
    }
}

impl Handler<PlayerId> for Player {
    type Result = ();

    fn handle(&mut self, msg: PlayerId, ctx: &mut Self::Context) {
        self.id = Some(msg.0);

        let socket_msg = OutSocketMessage::Id {
            id: self.id.unwrap(),
        };

        ctx.text(serde_json::to_string(&socket_msg).unwrap());
    }
}

impl Handler<TurnEnded> for Player {
    type Result = ();

    fn handle(&mut self, _: TurnEnded, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&OutSocketMessage::TurnEnded).unwrap());
    }
}

impl Handler<GameEnded> for Player {
    type Result = ();

    fn handle(&mut self, _: GameEnded, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&OutSocketMessage::GameEnded).unwrap());
    }
}
