use actix::prelude::{Message, Recipient};
use uuid::Uuid;
use serde::{Serialize, Deserialize};


// Message sent to the user
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct WebSocketMessage{
    pub msgType: OutgoingMessageType,
    pub data: MessageValue
}

#[derive(Serialize, Deserialize)]
pub enum OutgoingMessageType {
    Roster,
    Msg,
    UserInfo
}

#[derive(Serialize, Deserialize)]
pub enum MessageValue {
    Message(TextMessage),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct TextMessage {
    pub sender: String,
    pub senderId: String,
    pub message: String,
    pub time: String
}

//When a user connects, it sends this message
#[derive(Message)]
#[rtype(result= "()")]
pub struct Connect {
    pub addr: Recipient<WebSocketMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
    pub username: String
}

//When a user disconnects, it sends this message
#[derive(Message)]
#[rtype(result="()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub id: Uuid
}

//Message from client to lobby
#[derive(Message)]
#[rtype(result="()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}


//messages for signal server

#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result="()")]
pub struct SignalMessage {
    pub MsgType: SDPType
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SDPType {
    Signal(Signal),
    Roster(Vec<String>),
    UserInfo(String)
}

#[derive(Message, Serialize, Deserialize, Clone, Debug)]
#[rtype(result="()")]
pub struct Signal {
    pub user_id: Uuid,
    pub signal: String,
    pub from_user_id: Uuid
}

#[derive(Message)]
#[rtype(result="()")]
pub struct SignalConnect {
    pub user_id: Uuid,
    pub username: String,
    pub addr: Recipient<SignalMessage>,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct SignalDisconnect {
    pub user_id: Uuid 
}