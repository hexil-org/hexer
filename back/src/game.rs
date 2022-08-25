use actix::{Actor, ActorContext, Addr, Context, Handler, Message, Supervised};
use rand::seq::SliceRandom;

use crate::player::{self, Player};

pub struct Game {
    players: Vec<Addr<Player>>,
    current_player: u8,
}

#[derive(Message)]
#[rtype("()")]
pub struct Connect {
    pub player: Addr<Player>,
}

#[derive(Message)]
#[rtype("()")]
pub struct Disconnect;

#[derive(Message)]
#[rtype("()")]
pub struct EndTurn {
    pub player_id: u8,
}

impl Actor for Game {
    type Context = Context<Self>;
}

impl Supervised for Game {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        *self = Game::new();
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            current_player: 0,
        }
    }

    /// Starts the game
    pub fn start_game(&mut self) {
        self.players.shuffle(&mut rand::thread_rng());

        for (idx, player) in self.players.iter().enumerate() {
            player.do_send(player::PlayerId(idx as u8));
        }
    }
}

impl Handler<Connect> for Game {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        const PLAYER_THRESHOLD: usize = 3;

        if self.players.len() >= PLAYER_THRESHOLD {
            // The game is already full, do nothing for now
            return;
        }

        self.players.push(msg.player);

        if self.players.len() == PLAYER_THRESHOLD {
            self.start_game();
        }
    }
}

impl Handler<EndTurn> for Game {
    type Result = ();

    fn handle(&mut self, msg: EndTurn, _: &mut Context<Self>) -> Self::Result {
        assert_eq!(msg.player_id, self.current_player);

        self.current_player = (self.current_player + 1) % 3;

        for player in &self.players {
            player.do_send(player::TurnEnded);
        }
    }
}

impl Handler<Disconnect> for Game {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, ctx: &mut Context<Self>) {
        for player in &self.players {
            player.do_send(player::GameEnded);
        }

        ctx.stop();
    }
}
