use actix::{fut, Actor, Addr, Handler, Message, Running, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::messages::{Connect, Disconnect, GameMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Represents a player
struct Player {
    /// The unique ID assigned to the player by the server
    id: usize,
    /// The address of the game this player is playing
    game: Addr<Game>,
    /// The heartbeat of this player (used for making sure that the player is not dead)
    heartbeat: Instant,
}

impl Player {
    /// Creates a new player
    pub fn new(id: u32, game: Addr<Game>) -> Player {
        Player {
            id,
            game,
            heartbeat: Instant::now(),
        }
    }

    fn heartbeat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |player, context| {
            if Instant::now().duration_since(player.heartbeat) > CLIENT_TIMEOUT {
                // The client has unexpectedly dropped their connection (i.e. they haven't PONG'ed back in CLIENT_TIMEOUT seconds)
                player.game.do_send(Disconnect {
                    player_id: player.id,
                });
                context.stop();

                return;
            }

            // Ping the player to make sure their still alive
            context.ping(b"");
        });
    }
}

impl Actor for Player {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start
    /// We register the player with the game
    fn started(&mut self, context: &mut Self::Context) {
        // Start the heartbeat loop
        self.heartbeat(context);

        // Tell the game that we have connected
        let connect_message = Connect {
            addr: context.address().recipient(),
            player_id: self.id,
        };
        let future = self.game.send(connect_message).into_actor(self);

        future
            .then(|result, _, context| {
                match result {
                    Ok(_) => (),
                    // Something is wrong with the game
                    _ => context.stop(),
                }

                fut::ready(())
            })
            .wait(context);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // Notify the game that the player has left
        self.game.do_send(Disconnect { player_id: self.id });

        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Player {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => {
                // Reset the heartbeat and send a ping back
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                // Reset the heartbeat
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                unimplemented!();
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
            ws::ProtocolError => ctx.stop(),
        }
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for Player {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
