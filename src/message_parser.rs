use nom::{
    branch::alt,
    bytes::streaming::{tag, take_while},
    character::streaming::{satisfy, space1, alphanumeric1},
    combinator::opt,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

pub struct RawIrcMessage<'a> {
    pub prefix: Option<&'a str>,
    pub command: &'a str,
    pub parameters: Vec<&'a str>,
}

fn lf_or_crlf(input: &str) -> IResult<&str, &str> {
    alt((tag("\r\n"), tag("\n")))(input)
}

fn prefix(input: &str) -> IResult<&str, &str> {
    nom::sequence::delimited(tag(":"), take_while(|c| c != ' '), tag(" "))(input)
}

fn middle_parameter_first_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c != ' ' && c != ':' && c != '\r' && c != '\n')(input)
}

fn middle_parameter_remaining(input: &str) -> IResult<&str, &str> {
    take_while(|c| c != ' ' && c != '\r' && c != '\n')(input)
}

fn middle_parameter(input: &str) -> IResult<&str, &str> {
    let (input_remaining, (first, remaining)) =
        tuple((middle_parameter_first_char, middle_parameter_remaining))(input)?;

    Ok((
        input_remaining,
        &input[..first.len_utf8() + remaining.len()],
    ))
}

fn command(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn trailing_parameter(input: &str) -> IResult<&str, &str> {
    preceded(tag(":"), take_while(|c| c != '\n' && c != '\r'))(input)
}

pub fn parse_message<'a>(input: &'a str) -> IResult<&str, RawIrcMessage<'a>> {
    let (input, prefix) = opt(prefix)(input)?;
    let (input, command) = command(input)?;
    let (input, _) = space1(input)?;
    let (input, mut parameters) = separated_list0(tag(" "), middle_parameter)(input)?;
    let (input, _) = opt(space1)(input)?;
    let (input, trailing) = opt(trailing_parameter)(input)?;
    let (input, _) = lf_or_crlf(input)?;

    if let Some(trailing) = trailing {
        parameters.push(trailing)
    }

    Ok((
        input,
        RawIrcMessage {
            prefix,
            command,
            parameters,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let (_, message) = parse_message("CAP LS 302\r\n").unwrap();
        assert_eq!("CAP", message.command);
        assert_eq!(message.parameters, ["LS", "302"]);
    }

    #[test]
    fn test_parse_message2() {
        let (_, message) = parse_message(":xxx CAP LS 302\r\n").unwrap();
        assert_eq!(Some("xxx"), message.prefix);
        assert_eq!("CAP", message.command);
        assert_eq!(message.parameters, ["LS", "302"]);
    }

    #[test]
    fn test_parse_message1() {
        let (_, message) = parse_message("USER Magno 0 * :...\r\n").unwrap();

        assert_eq!("USER", message.command);
        assert_eq!(message.parameters, ["Magno", "0", "*", "..."]);        
    }

    #[test]
    fn test_parse_message3() {
        let (_, message) = parse_message("PING :TIMEOUTCHECK\r\n").unwrap();

        assert_eq!("PING", message.command);
        assert_eq!(message.parameters, ["TIMEOUTCHECK"]);
    }    
}