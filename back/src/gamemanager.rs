use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};

use crate::messages::{Connect, Disconnect, GameMessage};

pub struct GameManager {
    player_addresses: HashMap<usize, Recipient<GameMessage>>,
}

impl GameManager {
    /// Returns a new instance of the game manager
    pub fn new() -> GameManager {
        GameManager {
            player_addresses: HashMap::new(),
        }
    }
}

impl Actor for GameManager {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handles the "Connect" message
impl Handler<Connect> for GameManager {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.player_addresses.len();
        self.player_addresses.insert(id, msg.client_manager_addr);

        id
    }
}

/// Handles the "Disconnect" message
impl Handler<Disconnect> for GameManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.player_addresses.remove(&msg.client_manager_id);
    }
}
