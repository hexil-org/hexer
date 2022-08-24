use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::gamemanager;
use crate::messages;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ClientManager {
    /// The identifier to identify this client manager.
    pub id: usize,
    /// The timestamp of the last heartbeat. The client must
    /// respond to our ping every CLIENT_TIMEOUT seconds,
    /// otherwise we will assume the connection is down.
    pub last_heartbeat: Instant,
    /// The address of the corresponding GameManager actor.
    pub game_manager_addr: Addr<gamemanager::GameManager>,
}

impl ClientManager {
    /// Returns a new instance of the game manager
    pub fn new(game_manager_addr: Addr<gamemanager::GameManager>) -> ClientManager {
        ClientManager {
            id: 0,
            last_heartbeat: Instant::now(),
            game_manager_addr,
        }
    }

    /// Periodically sends a ping to the client, and checks
    /// when the last heartbeat from the client was received.
    fn heartbeat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.last_heartbeat) >= CLIENT_TIMEOUT {
                // The client has not sent back a heartbeat. Notify the game manager that the client has
                // disconnected and end the connection.
                actor.game_manager_addr.do_send(messages::Disconnect {
                    client_manager_id: actor.id,
                });
                context.stop();
            } else {
                // Send a ping
                context.ping(b"");
            }
        });
    }
}

impl Actor for ClientManager {
    type Context = ws::WebsocketContext<Self>;

    /// This function is called by actix when the actor has started.
    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);

        // Register the client manager to the global game manager
        // In the future, we'll likely have a "Lobby" process that manages game managers and creates
        // and assigns clients to game managers.
        self.game_manager_addr
            .send(messages::Connect {
                client_manager_addr: context.address().recipient(),
            })
            .into_actor(self)
            .then(|response, actor, context| {
                match response {
                    Ok(response) => actor.id = response,
                    _ => context.stop(),
                }

                fut::ready(())
            })
            .wait(context);
    }

    /// This function is called by actix when the actor is stopping.
    fn stopping(&mut self, context: &mut Self::Context) -> Running {
        self.game_manager_addr.do_send(messages::Disconnect {
            client_manager_id: self.id,
        });
        Running::Stop
    }
}

/// This implementation handles any incoming messages that have been sent
/// by another actor. For now, simply relay them to the WebSocket connection.
/// NOTE: This is not the handler for incoming WebSocket messages.
impl Handler<messages::GameMessage> for ClientManager {
    type Result = ();

    fn handle(&mut self, msg: messages::GameMessage, context: &mut Self::Context) {
        // Simply send the message to the WebSocket
        context.text(msg.0);
    }
}

/// Handle incoming WebSocket messages.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientManager {
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
                dbg!("Received ping");

                self.last_heartbeat = Instant::now();
                context.pong(&message);
            }
            Ok(ws::Message::Pong(_)) => {
                dbg!("Received pong");

                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Binary(_)) => println!("Unexpected binary"),
            Ok(ws::Message::Close(reason)) => {
                context.close(reason);
                context.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                context.stop();
            }
            Ok(ws::Message::Nop) => (),
            Err(_) => context.stop(),
        }
    }
}
