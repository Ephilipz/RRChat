use std::str::FromStr;

use crate::{
    messages::{Signal, SignalConnect, SignalDisconnect, SignalMessage},
    videolobby::VideoLobby,
};
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use serde_json::{json, Value};
use uuid::Uuid;

pub struct SignalSocket {
    user_id: Uuid,
    lobby: Addr<VideoLobby>,
    username: String,
}

impl SignalSocket {
    pub fn new(lobby: Addr<VideoLobby>, username: String) -> SignalSocket {
        SignalSocket {
            user_id: Uuid::new_v4(),
            lobby,
            username,
        }
    }
}

impl Actor for SignalSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.lobby
            .send(SignalConnect {
                addr: addr.recipient(),
                username: self.username.clone(),
                user_id: self.user_id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        self.lobby.do_send(SignalDisconnect {
            user_id: self.user_id,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SignalSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),

            Ok(ws::Message::Text(signal)) => {
                let signal: Signal = serde_json::from_str(&signal).unwrap();
                return self.lobby.do_send(signal);
            },

            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }

            //if it's a continuation, do nothing
            Ok(ws::Message::Continuation(_)) => (),

            Ok(ws::Message::Nop) => (),

            _ => (),
        }
    }
}

impl Handler<SignalMessage> for SignalSocket {
    type Result = ();

    fn handle(&mut self, msg: SignalMessage, ctx: &mut Self::Context) {
        ctx.text(json!(msg).to_string());
    }
}
