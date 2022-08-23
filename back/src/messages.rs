use actix::prelude::{Message, Recipient};

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<GameMessage>,
    pub player_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub player_id: usize,
}
