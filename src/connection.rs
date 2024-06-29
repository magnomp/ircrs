use std::net::SocketAddr;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream, sync::mpsc,
};

use crate::{client::Client, irc_message::IrcMessage, message_parser::parse_message, reply::IrcResponse};

pub struct Connection {
    reader: ReadHalf<TcpStream>,
    writer: WriteHalf<TcpStream>,
    client: Client,
}

impl Connection {
    pub fn new(reader: ReadHalf<TcpStream>, writer: WriteHalf<TcpStream>, client_addr: SocketAddr) -> Self {
        Connection { reader, writer, client: Client::new(client_addr.ip().to_string()) }
    }

    async fn send_response(&mut self, response: &IrcResponse) -> io::Result<()> {
        print!("> :blablaserver ");
        self.writer.write(":blablaserver ".as_bytes()).await?;
        print!("{}", &response.kind.to_irc_string());
        self.writer.write(&response.kind.to_irc_string().as_bytes()).await?;

        for n in 0..response.arguments.len() {
            self.writer.write(b" ").await?;
            print!(" ");
            if (n == response.arguments.len() - 1) {
                print!(":");
                self.writer.write(b":").await?;
            }
            print!("{}", response.arguments[n]);
            self.writer.write(response.arguments[n].as_bytes()).await?;
        }
        print!("\r\n");
        self.writer.write(b"\r\n").await?;        
        self.writer.flush().await?;

        Ok(())
    }
    pub fn spawn(mut self) {
        tokio::spawn(async move {
            let mut buffer = String::new();
            let mut temp_buffer = [0; 512];

            loop {
                match self.reader.read(&mut temp_buffer).await {
                    Ok(0) => {
                        break; // Connection closed
                    }
                    Ok(n) => {
                        if let Ok(new_data) = std::str::from_utf8(&temp_buffer[..n]) {
                            println!("<{}", new_data);
                            buffer.push_str(new_data)
                        } else {
                            eprintln!("received invalid UTF-8 sequence");
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        break;
                    }
                }

                while buffer.len() > 0 {
                    match parse_message(&buffer) {
                        Ok((remaining, parsed_message)) => {
                            let parsed_len = buffer.len() - remaining.len();
                            
                            match IrcMessage::from_raw(parsed_message) {                                
                                Ok(message) => {
                                    if let Some(messages) = self.client.handle(&message) {
                                        for message in messages.iter() {
                                            self.send_response(message).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    self.send_response(&e).await;
                                }
                            }        
                            buffer.drain(..parsed_len);       
                        }
                        Err(nom::Err::Incomplete(_)) => {
                            // Incomplete data, wait for more
                            break;
                        }
                        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                            // Error in parsing, handle or discard
                            println!("Parsing error: {}", e.input);
                            buffer.clear();
                            break;
                        }
                    }
                }
            }
        });
    }
}
