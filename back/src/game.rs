use actix::{Actor, Addr, Context, Handler, Message};

use crate::player::{self, Player};

pub struct Game {
    players: Vec<Addr<Player>>,
}

#[derive(Message)]
#[rtype("()")]
pub struct Connect {
    pub player: Addr<Player>,
}

#[derive(Message)]
#[rtype("()")]
pub struct Disconnect;

impl Actor for Game {
    type Context = Context<Self>;
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
        }
    }

    /// Starts the game
    pub fn start_game(&mut self) {
        for (idx, player) in self.players.iter().enumerate() {
            player.do_send(player::PlayerId(idx as u8));
        }
    }
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

impl Handler<Disconnect> for Game {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _: &mut Context<Self>) {
        // TODO: Stop game on disconnect
        unimplemented!()
    }
}
