use crate::{irc_message::IrcMessage, reply::IrcResponse};

pub struct Client {
    nick: Option<String>,
    username: Option<String>,
    client_addr: String,
    is_registered: bool
}

impl Client {
    pub fn new(client_addr: String) -> Client {
        println!("addr: {}", client_addr);
        Client {
            nick: None,
            username: None,
            client_addr,
            is_registered: false
        }
    }

    fn check_finished_registration(&mut self) -> Option<Vec<IrcResponse>> {
        if (self.is_registered) {
            return Some(vec!(IrcResponse::for_code(crate::reply::IrcResponseCode::ERR_ALREADYREGISTRED, vec!("Unauthorized command (already registered)".to_string()))))
        }
        let nick = self.nick.as_deref()?;
        let username = self.username.as_deref()?;
        self.is_registered = true;

        Some(vec!(IrcResponse::for_code(crate::reply::IrcResponseCode::RPL_WELCOME, vec!(nick.to_string(), format!("Welcome to the internet relay network {}!{}@{}", nick, username, self.client_addr).to_string()))))
    }

    fn handleCapLs(&mut self, version: &Option<u16>) -> Option<Vec<IrcResponse>> {
        // IRCv3 features will be implemented in the future
        None
    }

    fn handleNick(&mut self, nickname: &String) -> Option<Vec<IrcResponse>> {
        self.nick = Some(nickname.clone());
        if (!self.is_registered) {
            self.check_finished_registration();
        }
        None
    }

    fn handleUser(&mut self, user_name: &String, invisible: &bool, wallops: &bool, real_name: &String) -> Option<Vec<IrcResponse>> {
        self.username = Some(user_name.clone());
        self.check_finished_registration()
    }   

    pub fn handle(&mut self, message: &IrcMessage) -> Option<Vec<IrcResponse>> {
        match message {
            IrcMessage::CapLs { version } => self.handleCapLs(version),
            IrcMessage::Nick { nickname } => self.handleNick(nickname),
            IrcMessage::User { user_name, invisible, wallops, real_name } => self.handleUser(user_name, invisible, wallops, real_name)
        }
    }
}