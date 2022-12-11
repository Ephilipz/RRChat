use crate::messages::{SDPType, Signal, SignalConnect, SignalDisconnect, SignalMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::HashMap;
use uuid::Uuid;

type Socket = Recipient<SignalMessage>;

pub struct VideoUser {
    pub name: String,
    pub id: Uuid,
    pub socket: Socket,
}

pub struct VideoLobby {
    users: HashMap<Uuid, VideoUser>,
}

impl Default for VideoLobby {
    fn default() -> VideoLobby {
        VideoLobby {
            users: HashMap::new(),
        }
    }
}

impl Actor for VideoLobby {
    type Context = Context<Self>;
}

impl VideoLobby {
    fn send_roster(&self, to: &Uuid) {
        // self.users.values().for_each(|usr| {
        // usr.socket.do_send(SignalMessage {
        self.users.get(to).unwrap().socket.do_send(SignalMessage {
            MsgType: SDPType::Roster(
                self.users
                    .values()
                    .map(|user| user.id.to_string())
                    .collect(),
            ),
        });
        // });
    }
}

impl Handler<SignalDisconnect> for VideoLobby {
    type Result = ();

    fn handle(&mut self, msg: SignalDisconnect, _ctx: &mut Self::Context) -> Self::Result {
        let user = self.users.remove(&msg.user_id).unwrap();
        println!("removed user with id {}", user.id.to_string());
        // self.send_roster();
    }
}

impl Handler<SignalConnect> for VideoLobby {
    type Result = ();

    fn handle(&mut self, msg: SignalConnect, ctx: &mut Self::Context) -> Self::Result {
        let video_user = VideoUser {
            id: msg.user_id,
            name: msg.username,
            socket: msg.addr,
        };

        video_user.socket.do_send(SignalMessage {
            MsgType: SDPType::UserInfo(msg.user_id.to_string()),
        });
        self.users.insert(msg.user_id, video_user);
        self.send_roster(&msg.user_id);
    }
}

impl Handler<Signal> for VideoLobby {
    type Result = ();

    fn handle(&mut self, msg: Signal, _: &mut Self::Context) -> Self::Result {
        self.users
            .get(&msg.user_id)
            .unwrap()
            .socket
            .do_send(SignalMessage {
                MsgType: SDPType::Signal(msg),
            });
    }
}
