use crate::utils::{Codec, MessageHandler};
use std::net::TcpStream;
use std::time;
use std::io::ErrorKind;

pub struct IrcChatScraper<'a> {
    codec: Codec,
    reconnect_time: u64,
    auth: &'a str,
    channel: &'a str,
}

impl<'a> IrcChatScraper<'a> {
    const IRC_SERVER: &'a str = "irc.chat.twitch.tv:6667";

    pub fn connect(auth: &'a str, channel: &'a str) -> std::io::Result<IrcChatScraper<'a>> {
        print!("Connecting...");
        if let Ok(stream) = TcpStream::connect(IrcChatScraper::IRC_SERVER) {
            println!(" Success!");
            Ok(IrcChatScraper {
                codec: Codec::new(stream)?,
                reconnect_time: 0,
                auth,
                channel
            })
        } else {
            println!(" Failed!");
            return Err(std::io::Error::new(ErrorKind::Other, "s"))
        }

    }

    fn reconnect(&mut self) -> std::io::Result<()> {
        println!("Reconnecting...");
        std::thread::sleep(time::Duration::from_secs(self.reconnect_time));
        if let Ok(stream) = TcpStream::connect(IrcChatScraper::IRC_SERVER) {
            self.codec = Codec::new(stream)?;
        } else {
            self.reconnect_time += 1;
            self.reconnect()?;
        }
        Ok(())
    }

    fn init_irc(&mut self) -> std::io::Result<()> {
        self.codec.send(&*format!("PASS {}\n", self.auth))?;
        self.codec.send(&*format!("NICK scraper\n"))?;
        self.codec.send(&*format!("JOIN #{}\n", self.channel))?;

        /*Receives the first 10 lines of messages from the IRC
        so that when scraping starts, the messages to be received are
        the twitch chat messages.
         */
        for _ in 0..10 {
            let msg = self.codec.receive()?;
            if msg.is_empty() {
                break;
            }
        }

        println!("Now scraping twitch.tv/{} chat",self.channel);
        Ok(())
    }

    pub fn scrape(&mut self) -> std::io::Result<()> {
        self.init_irc()?;

        loop {
            let raw_message = self.codec.receive()?;

            if raw_message.is_empty() {
                self.reconnect()?;
                continue
            }

            if raw_message.starts_with("PING") {
                self.codec.send("PONG\n")?;
                continue
            }

            let trimmed_msg = MessageHandler::filter_irc(&raw_message);
            println!("{}", trimmed_msg);
        }

        Ok(())
    }
}
