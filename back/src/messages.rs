use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub client_manager_addr: Recipient<GameMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub client_manager_id: usize,
}
