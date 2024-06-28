use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream, sync::mpsc,
};

use crate::{client::Client, irc_message::IrcMessage, message_parser::parse_message, reply::IrcResponse};

pub struct Connection {
    reader: ReadHalf<TcpStream>,
    writer: WriteHalf<TcpStream>,
    client: Client,
}

impl Connection {
    pub fn new(reader: ReadHalf<TcpStream>, writer: WriteHalf<TcpStream>) -> Self {
        Connection { reader, writer, client: Client::new() }
    }

    async fn send_response(&mut self, response: IrcResponse) {
        self.writer.write(":blablaserver ".as_bytes());
        match response.kind {
            crate::reply::IrcResponseKind::Code(code) => self.writer.write(format!("{:0>3}", u16::from(code)).as_bytes()),
            crate::reply::IrcResponseKind::Command(command) => self.writer.write(command.as_bytes())
        };        
        
        for n in 0..response.arguments.len() {
            self.writer.write(b" ");
            if (n == response.arguments.len() - 1) {
                self.writer.write(b":");
            }
            self.writer.write(response.arguments[n].as_bytes());
        }
        self.writer.write(b"\r\n");
        self.writer.flush().await;
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

                            println!(
                                "{} - {:?}",
                                parsed_message.command, parsed_message.parameters
                            );
                            let response: IrcResponse;
                            match IrcMessage::from_raw(parsed_message) {                                
                                Ok(message) => {
                                    response = self.client.handle(&message);
                                }
                                Err(e) => {
                                    response = e;
                                }
                            }        
                            buffer.drain(..parsed_len);       
                            self.send_response(response);
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
