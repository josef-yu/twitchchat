use clap::ArgMatches;

static AUTH: &str = "o96k9izwlxp3mqtdtj5mq2632cm5n5";

pub enum Operation<'a> {
    ChannelChat(&'a str),
    VodChat(&'a str),
    Token(Option<&'a str>)
}

impl Operation<'_> {
    pub fn get_value(&self) -> Option<&str> {
        match self {
            Operation::Token(Some(x)) => Some(x),
            Operation::Token(None) => None,
            Operation::ChannelChat(x) => Some(x),
            Operation::VodChat(x) => Some(x),
        }
    }
}

pub struct Config<'a> {
    pub auth: &'a str,
    pub operation: Operation<'a>,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a ArgMatches) -> Result<Config<'a>, &'static str> {
        let channel = args.value_of("CHANNEL");
        let vod_id = args.value_of("VOD_ID");
        let token = args.value_of("TOKEN");

        let operation = match (channel, vod_id, token) {
            (Some(x), None, None) =>  Operation::ChannelChat(x),
            (None, Some(x), None) => Operation::VodChat(x),
            (None, None, Some(x)) => Operation::Token(Option::from(x)),
            _=> return Err("Only one required option at a time. Use --help flag for usage info.")
        };

        //TODO get token via file
        let auth = AUTH;

        Ok(Config {auth, operation})
    }
}