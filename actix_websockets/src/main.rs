mod websocket;
mod lobby;
mod signalsocket;
mod videolobby;
use lobby::Lobby;
mod messages;
mod start_connection;
use openssl::ssl::SslAcceptor;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use start_connection::start_connection as start_connection_route;
use start_connection::start_sdp_connection as start_sdp_route;
use actix::Actor;

use actix_web::{App, HttpServer, web::Data};
use videolobby::VideoLobby;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start(); //create and spin up a lobby
    let video_server = VideoLobby::default().start();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
            .set_private_key_file("cert/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert/cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route) //chat websocket
            .service(start_sdp_route) //webrtc signal websocket
            .app_data(Data::new(chat_server.clone())) //register the lobby
            .app_data(Data::new(video_server.clone())) //register video server
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    // .bind("172.20.10.5:8080")?
    .run()
    .await
}
