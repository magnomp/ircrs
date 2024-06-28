mod message_parser;
mod connection;
mod irc_message;
mod client;
mod reply;

use connection::Connection;
use tokio::net::TcpListener;

use std::io;
use tokio::io::split;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6667")
        .await
        .expect("Falha ao iniciar servidor");
    println!("Start listening");
    loop {
        let (socket, _) = listener.accept().await.expect("Falha ao aceitar");
        println!("Accepted");
        let (reader, writer) = split(socket);

        let connection = Connection::new(reader, writer);
        connection.spawn();
    }
}