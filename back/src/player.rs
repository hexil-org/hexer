use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

use crate::game;

const SIGN_OF_LIFE_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Player {
    /// The timestamp of the last sign of life. The client must
    /// respond to our ping every CLIENT_TIMEOUT seconds,
    /// otherwise we will assume the connection is down.
    pub last_sign_of_life: Instant,
    pub game: Addr<game::Game>,
}

#[derive(Message)]
#[rtype("()")]
pub struct PlayerId(pub u8);

#[derive(Serialize)]
#[serde(tag = "t", rename_all = "camelCase")]
enum SocketMessage {
    Id { id: u8 },
}

impl Player {
    pub fn new(game: Addr<game::Game>) -> Player {
        Player {
            last_sign_of_life: Instant::now(),
            game,
        }
    }

    /// Periodically sends a ping to the client, and checks
    /// when the last sign of life from the client was received.
    fn setup_last_sign_of_life_check(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(SIGN_OF_LIFE_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.last_sign_of_life) >= CLIENT_TIMEOUT {
                // The client has not given a sign of life.
                context.stop();
            } else {
                context.ping(b"");
            }
        });
    }
}

impl Actor for Player {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.setup_last_sign_of_life_check(context);

        // Register the player to the global game.
        self.game.do_send(game::Connect {
            player: context.address(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.game.do_send(game::Disconnect);
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Player {
    fn handle(
        &mut self,
        message: Result<ws::Message, ws::ProtocolError>,
        context: &mut Self::Context,
    ) {
        match message {
            Ok(ws::Message::Text(text)) => {
                dbg!(text);
            }
            Ok(ws::Message::Ping(message)) => {
                self.last_sign_of_life = Instant::now();
                context.pong(&message);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_sign_of_life = Instant::now();
            }
            Ok(ws::Message::Close(reason)) => {
                context.close(reason);
                context.stop();
            }
            Ok(_) => (),
            Err(_) => context.stop(),
        }
    }
}

impl Handler<PlayerId> for Player {
    type Result = ();

    fn handle(&mut self, msg: PlayerId, context: &mut Self::Context) {
        let socket_message = SocketMessage::Id { id: msg.0 };

        context.text(serde_json::to_string(&socket_message).unwrap())
    }
}
