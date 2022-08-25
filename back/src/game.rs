use actix::{Actor, Addr, Context, Handler, Message};

use crate::player::Player;

pub struct Game {
    has_started: bool,
    players: Vec<Addr<Player>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            has_started: false,
            players: Vec::new(),
        }
    }

    /// Starts the game
    pub fn start_game(&mut self) {
        self.has_started = true;

        // TODO: Send the players their ID
    }
}

impl Actor for Game {
    type Context = Context<Self>;
}

pub struct Connect {
    pub player: Addr<Player>,
}

impl Message for Connect {
    type Result = ();
}

impl Handler<Connect> for Game {
    type Result = ();

    fn handle(&mut self, message: Connect, _: &mut Context<Self>) -> Self::Result {
        const PLAYER_THRESHOLD: usize = 3;

        if self.players.len() >= PLAYER_THRESHOLD {
            // The game is already full, do nothing for now
            return;
        }

        self.players.push(message.player);

        if self.players.len() == PLAYER_THRESHOLD {
            self.start_game();
        }
    }
}

pub struct Disconnect;

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Disconnect> for Game {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _: &mut Context<Self>) {
        // TODO: Stop game on disconnect
        unimplemented!()
    }
}
