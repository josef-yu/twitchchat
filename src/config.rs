use clap::ArgMatches;
use crate::credentials::Credentials;

#[derive(PartialEq)]
pub enum Operation<'a> {
    ChannelChat(&'a str),
    VodChat(&'a str),
    Token(Option<&'a str>),
    ClientID(Option<&'a str>)
}

impl Operation<'_> {
    pub fn get_value(&self) -> Option<&str> {
        match self {
            Operation::Token(Some(x)) => Some(x),
            Operation::Token(None) => None,
            Operation::ClientID(Some(x)) => Some(x),
            Operation::ClientID(None) => None,
            Operation::ChannelChat(x) => Some(x),
            Operation::VodChat(x) => Some(x),
        }
    }
}

pub struct Config<'a> {
    pub credentials: Credentials,
    pub operation: Operation<'a>,
    pub log_filename: Option<&'a str>,
    pub should_log: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a ArgMatches) -> Result<Config<'a>, &'static str> {
        let channel = args.value_of("CHANNEL");
        let vod_id = args.value_of("VOD_ID");
        let token = args.value_of("TOKEN");
        let arg_client_id = args.value_of("CLIENTID");
        let log_filename = args.value_of("FILENAME");
        let should_log = args.is_present("log");

        let operation = match (channel, vod_id, token, arg_client_id) {
            (Some(x), None, None, None) =>  Operation::ChannelChat(x),
            (None, Some(x), None, None) => Operation::VodChat(x),
            (None, None, Some(x), None) => Operation::Token(Option::from(x)),
            (None, None, None, Some(x)) => Operation::ClientID(Option::from(x)),
            _=> return Err("Only one required option at a time. Use --help flag for usage info.")
        };

        let credentials = Credentials::get_credentials()?;

        if token.is_none() && arg_client_id.is_none(){
            credentials.check_credentials()?;
        }


        Ok(Config {
            credentials,
            operation,
            log_filename,
            should_log
        })
    }
}