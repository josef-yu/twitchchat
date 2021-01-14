use crate::utils::Codec;
use std::net::TcpStream;
use crate::config::Config;
use regex::Regex;
use gh_emoji::Replacer;

static IRC_SERVER: &str = "irc.chat.twitch.tv:6667";

fn init_connection(auth: &str, channel: &str) -> std::io::Result<Codec>{
    println!("Connecting...");
    let stream = TcpStream::connect(IRC_SERVER)?;
    let mut codec = Codec::new(stream)?;

    codec.send(&*format!("PASS {}\n", auth))?;
    codec.send(&*format!("NICK scraper\n"))?;
    codec.send(&*format!("JOIN #{}\n", channel))?;

    for _ in 0..10 {
        let msg = codec.receive()?;
        if msg.is_empty() {
            break;
        }
    }

    println!("Now scraping twitch.tv/{} chat",channel);

    Ok(codec)
}

fn filter_msg(msg: &String) -> String {
    let cap = Regex::new(r":(.*)!.*@.*\.tmi\.twitch\.tv PRIVMSG #.* :(.*)").unwrap()
        .captures(msg).unwrap();
    let demojied_msg = Replacer::new().replace_all(&cap[2]);
    format!("{}: {}", &cap[1], demojied_msg.to_string())
}

pub fn scrape_irc(config: Config) -> std::io::Result<()> {
    let mut codec = init_connection(config.auth, config.operation.get_value().unwrap())?;

    loop {
        let raw_message = codec.receive()?;

        if raw_message.is_empty() {
            codec = init_connection(config.auth, config.operation.get_value().unwrap())?;
            continue
        }

        if raw_message.starts_with("PING") {
            codec.send("PONG\n")?;
            continue
        }

        let trimmed_msg = filter_msg(&raw_message);
        println!("{}", trimmed_msg);
    }

    Ok(())
}
