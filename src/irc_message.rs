use crate::{message_parser::RawIrcMessage, reply::{IrcResponse, IrcResponseCode}};

#[derive(Debug)]
pub enum IrcMessage {
    CapLs {
        version: Option<u16>,
    },
    Nick {
        nickname: String,
    },
    User {
        user_name: String,
        invisible: bool,
        wallops: bool,
        real_name: String,
    },
}

impl IrcMessage {
    fn cap(raw: RawIrcMessage) -> Result<IrcMessage, IrcResponse> {
        match raw.parameters[..] {
            ["LS"] => Ok(IrcMessage::CapLs { version: None }),            
            ["LS", version, ..] => match str::parse::<u16>(version) {                
                Ok(version) => Ok(IrcMessage::CapLs {
                    version: Some(version),
                }),
                _ => Err(IrcResponse::for_code(IrcResponseCode::ERR_GENERIC, vec!("CAP LS version argument should be a numeric string".to_string()))),
            },
            [subcommand, ..] => Err(IrcResponse::for_code(IrcResponseCode::ERR_INVALIDCAPCMD, vec!("*".to_string(), subcommand.to_string(), "Invalid CAP command".to_string()))),
            [] => Err(IrcResponse::for_code(IrcResponseCode::ERR_UNKNOWNCOMMAND, vec!("Must specify subcommand to CAP".to_string()))),

        }
    }

    fn nick(raw: RawIrcMessage) -> Result<IrcMessage, IrcResponse> {
        match raw.parameters[..] {
            [nickname, ..,] => Ok(IrcMessage::Nick {
                nickname: nickname.to_string(),
            }),
            [] => Err(IrcResponse::for_code(IrcResponseCode::ERR_NONICKNAMEGIVEN, vec!("No nickname given".to_string()))),
        }
    }

    fn user(raw: RawIrcMessage) -> Result<IrcMessage, IrcResponse> {
        match raw.parameters[..] {
            [user_name, mode, _, real_name, ..] => match str::parse::<u8>(mode) {
                Ok(mode) => Ok(IrcMessage::User {
                    user_name: user_name.to_string(),
                    invisible: mode & 0b1000 != 0,
                    wallops: mode & 0b100 != 0,
                    real_name: real_name.to_string(),
                }),
                Err(_) => Err(IrcResponse::for_code(IrcResponseCode::ERR_GENERIC, vec!("USER mode argument should be a numeric string".to_string()))),
            },
            _ => Err(IrcResponse::for_code(IrcResponseCode::ERR_NEEDMOREPARAMS, vec!("USER".to_string(), "Not enough parameters".to_string()))),
        }
    }

    pub fn from_raw(raw: RawIrcMessage) -> Result<IrcMessage, IrcResponse> {
        match raw.command {
            "CAP" => IrcMessage::cap(raw),
            "NICK" => IrcMessage::nick(raw),
            "USER" => IrcMessage::user(raw),
            command => Err(IrcResponse::for_code(IrcResponseCode::ERR_UNKNOWNCOMMAND, vec!(command.to_string(), "Unknown command".to_string()))),
        }
    }
}
