use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::game;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Player {
    /// The timestamp of the last heartbeat. The client must
    /// respond to our ping every CLIENT_TIMEOUT seconds,
    /// otherwise we will assume the connection is down.
    pub last_heartbeat: Instant,
    /// The address of the corresponding GameManager actor.
    pub game: Addr<game::Game>,
}

impl Player {
    pub fn new(game: Addr<game::Game>) -> Player {
        Player {
            last_heartbeat: Instant::now(),
            game,
        }
    }

    /// Periodically sends a ping to the client, and checks
    /// when the last heartbeat from the client was received.
    fn heartbeat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.last_heartbeat) >= CLIENT_TIMEOUT {
                // The client has not sent back a heartbeat. Notify the game manager that the client has
                // disconnected and end the connection.
                actor.game.do_send(game::Disconnect);
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
        self.heartbeat(context);

        // Register the client manager to the global game. In the
        // future, we'll likely have a "Lobby" process that manages game
        // managers and creates and assigns clients to game managers.
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
                self.last_heartbeat = Instant::now();
                context.pong(&message);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
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
