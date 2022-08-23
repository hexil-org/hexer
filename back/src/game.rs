use crate::messages::{Connect, Disconnect, GameMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::HashMap;

pub struct Game {
    players: HashMap<usize, Recipient<GameMessage>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
        }
    }
}

impl Game {
    /// Send message to a player
    fn send_message(&self, player_id: usize, message: &str) {
        let addr = self.players.get(&player_id).unwrap();
        addr.do_send(GameMessage(message.to_owned()))
    }
}

impl Actor for Game {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<Connect> for Game {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        let id = self.players.keys().len();
        self.players.insert(id, msg.addr);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}
