pub mod codec;
pub mod config;
pub mod twitch;
pub mod credentials;
pub mod logger;
pub mod message;
mod irc_chat;

use clap::{ArgMatches, App, Arg};
use std::error::Error;
use crate::config::{Config, Operation};
use crate::irc_chat::IrcChatScraper;


pub fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("Twitch Chat Scraper")
        .version("0.5.0")
        .about("Scrapes twitch chat from vod, live stream, or offline stream")
        .usage("twitchchat.exe [FLAGS] [OPTIONS]")
        .arg(Arg::with_name("CHANNEL")
            .short("c")
            .long("channel")
            .takes_value(true)
            .required_unless_one(&["VOD_ID", "TOKEN", "CLIENTID"])
            .help("Twitch channel chat to scrape"))
        .arg(Arg::with_name("VOD_ID")
            .short("v")
            .long("vod")
            .takes_value(true)
            .required_unless_one(&["CHANNEL", "TOKEN", "CLIENTID"])
            .help("Twitch VOD ID to scrape"))
        .arg(Arg::with_name("TOKEN")
            .short("t")
            .long("token")
            .takes_value(true)
            .required_unless_one(&["VOD_ID", "CHANNEL", "CLIENTID"])
            .help("Sets IRC token authentication\nDisplays token if value is not provided\nUse --help for more info")
            .long_help("The OATH token to be used can be obtained via https://twitchapps.com/tmi/\n\
            You must have a twitch account to obtain the said token."))
        .arg(Arg::with_name("CLIENTID")
            .short("i")
            .long("clientid")
            .takes_value(true)
            .required_unless_one(&["VOD_ID", "CHANNEL", "TOKEN"])
            .help("Sets client ID for the app. Displays token if value is not provided\nUse --help for more info")
            .long_help("The client ID to be used can be obtained via https://dev.twitch.tv\n\
            You must have a twitch account to obtain the said client id."))
        .arg(Arg::with_name("log")
            .short("l")
            .long("log")
            .takes_value(true)
            .value_name("FILENAME")
            .min_values(0)
            .help("Saves chat to text file"))
        .arg(Arg::with_name("live") //TODO implementation
            .long("live")
            .multiple(false)
            .help("Scrape from live stream chat\nExits if offline"))
        .get_matches()
}

pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    match config.operation {
        Operation::Token(x) => {
            config.credentials.auth = String::from(x.unwrap());
            config.credentials.save_credentials()?;
        },
        Operation::VodChat(_) => {},
        Operation::ClientID(x) => {
            config.credentials.client_id = String::from(x.unwrap());
            config.credentials.save_credentials()?;
        },
        Operation::ChannelChat(_) => {
            IrcChatScraper::connect(&config.credentials, config.log_filename, config.should_log, config.operation.get_value().unwrap())?
                .scrape()?;
        },
    }

    Ok(())

}


