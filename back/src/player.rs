use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

use crate::game;

const SIGN_OF_LIFE_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Player {
    pub id: Option<u8>,
    /// The timestamp of the last sign of life. The client must
    /// respond to our ping every CLIENT_TIMEOUT seconds,
    /// otherwise we will assume the connection is down.
    pub last_sign_of_life: Instant,
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
        self.setup_last_sign_of_life_check(ctx);

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
            last_sign_of_life: Instant::now(),
            game,
        }
    }

    /// Periodically sends a ping to the client, and checks
    /// when the last sign of life from the client was received.
    fn setup_last_sign_of_life_check(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(SIGN_OF_LIFE_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.last_sign_of_life) >= CLIENT_TIMEOUT {
                // The client has not given a sign of life.
                context.stop();
            } else {
                context.ping(b"");
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Player {
    fn handle(
        &mut self,
        maybe_msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match maybe_msg {
            Ok(msg) => {
                self.last_sign_of_life = Instant::now();

                match msg {
                    ws::Message::Text(text) => {
                        let socket_msg: InSocketMessage = serde_json::from_str(&text).unwrap();

                        match socket_msg {
                            InSocketMessage::EndTurn => {
                                // TODO: Don't unwrap
                                self.game.do_send(game::EndTurn {
                                    player_id: self.id.unwrap(),
                                });
                            }
                        }
                    }
                    ws::Message::Ping(bytes) => {
                        ctx.pong(&bytes);
                    }
                    ws::Message::Close(reason) => {
                        ctx.close(reason);
                        ctx.stop();
                    }
                    _ => (),
                }
            }
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
