use crate::messages::{
    ClientActorMessage, Connect, Disconnect, MessageValue, OutgoingMessageType, TextMessage,
    WebSocketMessage,
};
use actix::prelude::{Actor, Context, Handler, Recipient};
use chrono::Utc;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

type Socket = Recipient<WebSocketMessage>;

pub struct ConnectedUser {
    pub id: Uuid,
    pub name: String,
    pub socket: Socket,
}

pub struct Lobby {
    // map from lobby id to socket
    sessions: HashMap<Uuid, ConnectedUser>,

    // map from room id to list of user ids
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

// default values for lobby
impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, sender: &Uuid, id_to: &Uuid) {
        //get recipient address
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            if let Some(sender) = self.sessions.get(sender) {
                let _ = socket_recipient.socket.do_send(WebSocketMessage {
                    msgType: OutgoingMessageType::Msg,
                    data: MessageValue::Message(TextMessage {
                        message: message.to_owned(),
                        sender: sender.name.clone(),
                        senderId: sender.id.to_string(),
                        time: Utc::now().to_string(),
                    }),
                });
                return;
            }
            panic!("can't find user id to send the message from");
        }
        panic!("can't find user id to send the message to");
    }

    fn send_roster(&self, room_id: &Uuid, except: Option<&Uuid>) {
        let room = self.rooms.get(room_id).unwrap();

        let roster: Vec<String> = room
            .iter()
            .map(|user| self.sessions.get(user).unwrap().name.clone())
            .collect();

        room.iter().for_each(|user| {
            if let Some(except_id) = except {
                if except_id == user {
                    return;
                }
            }
            let socket = &self.sessions.get(user).unwrap().socket;
            (*socket).do_send(WebSocketMessage {
                msgType: OutgoingMessageType::Roster,
                data: MessageValue::String(json!(roster).to_string()),
            });
        })
    }

    // sends the client his id
    fn send_userInfo(&self, user: &ConnectedUser) {
        user.socket.do_send(WebSocketMessage {
            msgType: OutgoingMessageType::UserInfo,
            data: MessageValue::String(user.id.to_string()),
        });
    }

}

impl Actor for Lobby {
    type Context = Context<Self>;
}

// Handler for disconnect messages
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if let Some(_) = self.sessions.remove(&msg.id) {
            // check if lobby contains the room and assign it to room
            if let Some(rooms) = self.rooms.get_mut(&msg.room_id) {
                if rooms.len() > 1 {
                    rooms.remove(&msg.id);
                    self.send_roster(&msg.room_id, None);
                    return;
                }

                // the user was the last in the room. Remove the room
                self.rooms.remove(&msg.room_id);
            }
        }
    }
}

// Handler for connect messages
impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // create a room if it doesn't exist, then add the id
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        let connected_user = ConnectedUser {
            id: msg.self_id,
            name: msg.username,
            socket: msg.addr,
        };

        self.send_userInfo(&connected_user);

        //store the newly joined user
        self.sessions.insert(msg.self_id, connected_user);

        self.send_roster(&msg.lobby_id, None);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        let is_whisper = msg.msg.starts_with("\\w");

        if is_whisper {
            // get id of the client to whisper to
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &msg.id, &Uuid::parse_str(id_to).unwrap());
            }
            return;
        }

        // send the message to all clients
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(&msg.msg, &msg.id, client));
    }
}
