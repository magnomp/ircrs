use crate::{irc_message::IrcMessage, reply::IrcResponse};

pub struct Client {
    nick: Option<String>,
    username: Option<String>
}

impl Client {
    pub fn new() -> Client {
        Client {
            nick: None,
            username: None
        }
    }

    fn handleCapLs(&mut self, version: &Option<u16>) -> Option<Vec<IrcResponse>> {
        Some(vec!(IrcResponse::for_command("CAP".to_string(), vec!("*".to_string(), "LS".to_string()))))
    }

    fn handleNick(&mut self, nickname: &String) -> Option<Vec<IrcResponse>> {
        self.nick = Some(nickname.clone());
        None
    }

    fn handleUser(&mut self, user_name: &String, invisible: &bool, wallops: &bool, real_name: &String) -> Option<Vec<IrcResponse>> {
        self.username = Some(user_name.clone());
        None
    }   

    pub fn handle(&mut self, message: &IrcMessage) -> Option<Vec<IrcResponse>> {
        match message {
            IrcMessage::CapLs { version } => self.handleCapLs(version),
            IrcMessage::Nick { nickname } => self.handleNick(nickname),
            IrcMessage::User { user_name, invisible, wallops, real_name } => self.handleUser(user_name, invisible, wallops, real_name)
        }
    }
}