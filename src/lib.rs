pub mod utils;
pub mod config;
pub mod twitch;
mod irc_chat;

use clap::{ArgMatches, App, Arg};
use std::error::Error;
use crate::config::{Config, Operation};
use crate::irc_chat::IrcChatScraper;

static CLIENT_ID: &str = "b5abx04k71homlrz3v4tnuxraku4ux";

pub fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("Twitch Chat Scraper")
        .version("0.1.0")
        .about("Scrapes twitch chat from vod, live stream, or offline stream")
        .usage("twitchchat.exe [FLAGS] [OPTIONS]")
        .arg(Arg::with_name("CHANNEL")
            .short("c")
            .long("channel")
            .takes_value(true)
            .required_unless_one(&["VOD_ID", "TOKEN"])
            .help("Twitch channel chat to scrape"))
        .arg(Arg::with_name("VOD_ID")
            .short("v")
            .long("vod")
            .takes_value(true)
            .required_unless_one(&["CHANNEL", "TOKEN"])
            .help("Twitch VOD ID to scrape"))
        .arg(Arg::with_name("TOKEN")
            .short("t")
            .long("token")
            .takes_value(true)
            .required_unless_one(&["VOD_ID", "CHANNEL"])
            .help("Set IRC token authentication\nDisplays token if value is not provided\nUse --help for more info")
            .long_help("The OATH token to be used can be obtained via https://twitchapps.com/tmi/\n\
            You must have a twitch account to obtain the said token."))
        .arg(Arg::with_name("live")
            .long("live")
            .multiple(false)
            .help("Scrape from live stream chat\nExits if offline"))
        .arg(Arg::with_name("log")
            .long("log")
            .takes_value(true)
            .value_name("FILENAME")
            .help("Saves chat to text file"))
        .get_matches()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.operation {
        Operation::Token(_) => {},
        Operation::VodChat(_) => {},
        Operation::ChannelChat(_) => {
            IrcChatScraper::connect(config.auth, config.operation.get_value().unwrap())?
                .scrape()?;
        },
    }

    Ok(())

}


