use crate::lobby::Lobby;
use crate::videolobby::VideoLobby;
use crate::websocket::WebSocketConnection;
use crate::signalsocket::SignalSocket;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::{uuid, Uuid};

static room_id: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

#[get("/{username}")]
pub async fn start_connection(
    request: HttpRequest,
    stream: Payload,
    path: Path<String>,
    server: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let username = path.into_inner();
    // create a wsConnection to the lobby
    let websocket = WebSocketConnection::new(room_id, server.get_ref().clone(), username);

    // start the websocket
    let response = ws::start(websocket, &request, stream)?;

    return Ok(response);
}

#[get("/live/{username}")]
pub async fn start_sdp_connection(
    request: HttpRequest,
    stream: Payload,
    path: Path<String>,
    server: Data<Addr<VideoLobby>>,
) -> Result<HttpResponse, Error> {
    let username = path.into_inner();
    let websocket = SignalSocket::new(server.get_ref().clone(), username);

    let response = ws::start(websocket, &request, stream)?;

    return Ok(response);
}
