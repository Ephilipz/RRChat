use crate::lobby::Lobby;
use crate::messages::{ClientActorMessage, Connect, Disconnect, WebSocketMessage};
use actix::{fut, ActorContext, ActorFutureExt, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use serde_json::json;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocketConnection {
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
    username: String,
}

impl WebSocketConnection {
    //constructor for websocketconnection
    pub fn new(room: Uuid, lobby: Addr<Lobby>, username: String) -> WebSocketConnection {
        WebSocketConnection {
            id: Uuid::new_v4(),
            room,
            hb: Instant::now(),
            lobby_addr: lobby,
            username,
        }
    }

    //handle heartbeat check. Disconnect if no acknowledgement is received
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |user, ctx| {
            let is_client_connected = Instant::now().duration_since(user.hb) <= CLIENT_TIMEOUT;

            if is_client_connected {
                ctx.ping(b"");
                return;
            }

            //client disconnected
            println!("Heartbeat failed. Disconnecting");

            //send signal to lobby to disconnect user
            user.lobby_addr.do_send(Disconnect {
                id: user.id,
                room_id: user.room,
            });

            //end the current context which the actor lives in
            ctx.stop();
            return;
        });
    }
}

//turn web socket connection into an actor
impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    //create the actor
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();

        //connect to lobby
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.id,
                username: self.username.clone(),
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

    //destory the actor
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

//match incoming websocket messages from client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            //client pings
            Ok(ws::Message::Ping(msg)) => {
                //reset the hb since we know the client is alive
                self.hb = Instant::now();
                //respond with pong
                ctx.pong(&msg);
            }

            //client pongs
            Ok(ws::Message::Pong(_)) => {
                //reset the hb since we know the client is alive
                self.hb = Instant::now();
            }

            //realistically shouldn't happen. But if the message is binary send it to the context to be handled there
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),

            //client sends a close message
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }

            //if it's a continuation, do nothing
            Ok(ws::Message::Continuation(_)) => (),

            Ok(ws::Message::Nop) => (),

            //if a text is sent, create a client actor message and send it to the lobby
            Ok(ws::Message::Text(text)) => {
                self.lobby_addr.do_send(ClientActorMessage {
                    id: self.id,
                    msg: text.to_string(),
                    room_id: self.room,
                });
            }

            Err(e) => panic!("Error in ws message {e}"),
        }
    }
}

//handle outgoing messages to client
impl Handler<WebSocketMessage> for WebSocketConnection {
    type Result = ();

    fn handle(&mut self, msg: WebSocketMessage, ctx: &mut Self::Context) {
        ctx.text(json!(msg).to_string())
    }
}
