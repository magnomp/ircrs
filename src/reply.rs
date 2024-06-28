pub enum IrcResponseKind {
    Code(IrcResponseCode),
    Command(String)
}

pub struct IrcResponse {
    pub kind: IrcResponseKind,
    pub arguments: Vec<String>
}



#[allow(non_camel_case_types)]
pub enum IrcResponseCode {
    ERR_INVALIDCAPCMD,
    ERR_UNKNOWNCOMMAND,
    ERR_NONICKNAMEGIVEN,
    ERR_NEEDMOREPARAMS,
    // Error condition (mostly misuse of protocol) which I didn't find a mapping on RFC
    ERR_GENERIC
}

impl From<IrcResponseCode> for u16 {
    fn from(value: IrcResponseCode) -> Self {
        match value {
            IrcResponseCode::ERR_INVALIDCAPCMD => 410,
            IrcResponseCode::ERR_UNKNOWNCOMMAND => 421,
            IrcResponseCode::ERR_NONICKNAMEGIVEN => 431,
            IrcResponseCode::ERR_NEEDMOREPARAMS => 461,
            IrcResponseCode::ERR_GENERIC => 999
        }
    }
}

impl IrcResponse {
    pub fn for_code(code: IrcResponseCode, arguments: Vec<String>) -> IrcResponse {
        IrcResponse { kind: IrcResponseKind::Code(code), arguments }
    }

    pub fn for_command(command: String, arguments: Vec<String>) -> IrcResponse {
        IrcResponse { kind: IrcResponseKind::Command(command), arguments }
    }
}