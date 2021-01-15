use crate::utils::{Codec, MessageFilter};
use std::net::TcpStream;
use std::io::ErrorKind;
use std::time::Duration;
use rand::{thread_rng, Rng};
use crossterm::event::{read,KeyEvent, Event, KeyCode, KeyModifiers};
use crate::twitch::TwitchApiHandler;

pub struct IrcChatScraper<'a> {
    codec: Codec<TcpStream>,
    reconnect_time: u64,
    auth: &'a str,
    channel: &'a str,
}
impl<'a> IrcChatScraper<'a> {
    const IRC_SERVER: &'a str = "irc.chat.twitch.tv:6667";

    pub fn connect(auth: &'a str, channel: &'a str) -> std::io::Result<Self> {
        print!("Connecting...");
        if let Ok(stream) = TcpStream::connect(Self::IRC_SERVER) {
            println!(" Success!");
            stream.set_read_timeout(Some(Duration::from_secs(180)))
                .expect("Setting read timeout failed!");
            Ok(IrcChatScraper {
                codec: Codec::new(stream)?,
                reconnect_time: 0,
                auth,
                channel
            })
        } else {
            println!(" Failed!");
            return Err(std::io::Error::new(ErrorKind::Other, "Connection failure"))
        }

    }

    fn reconnect(&mut self) -> std::io::Result<()> {
        println!("Reconnecting...");
        std::thread::sleep(Duration::from_secs(self.reconnect_time));
        if let Ok(stream) = TcpStream::connect(Self::IRC_SERVER) {
            self.codec = Codec::new(stream)?;
            self.init_irc()?;
        } else {
            if self.reconnect_time >= 6 {
                return Err(std::io::Error::new(ErrorKind::Other, "Cannot reconnect to IRC server"));
            }
            self.reconnect_time += 1;
            self.reconnect()?;
        }
        Ok(())
    }

    fn continue_prompt(is_live: bool) -> bool {
        if !is_live {
            println!("This channel is offline. Do you want to continue? y/n");
            loop {
                match read().unwrap() {
                    Event::Key(KeyEvent{
                                   code: KeyCode::Char('y'),
                                   modifiers: KeyModifiers::NONE,
                               }) => return true,
                    Event::Key(KeyEvent{
                                   code: KeyCode::Char('n'),
                                   modifiers: KeyModifiers::NONE,
                               }) => return false,
                    _ => {}
                }
            }
        } else {
            true
        }
    }

    fn init_irc(&mut self) -> std::io::Result<()> {
        let is_live = TwitchApiHandler::is_live(self.channel)
            .expect("Failed to fetch stream info.");
        let continue_prompt = Self::continue_prompt(is_live);
        if !continue_prompt {
            return Err(std::io::Error::new(ErrorKind::Other, "Scraping aborted"))
        }

        self.codec.send(&*format!("PASS oauth:{}\n", self.auth))?;
        self.codec.send("NICK scraper\n")?;
        self.codec.send("CAP REQ :twitch.tv/tags\n")?;
        self.codec.send(&*format!("JOIN #{}\n", self.channel))?;

        /*Receives the first 11 lines of messages from the IRC server
        so that when scrape() is called, the messages to be received are
        the twitch chat messages.
         */
        for _ in 0..11 {
            let msg = self.codec.receive()?;
            if msg.is_empty() {
                self.reconnect()?;
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
            let (username, trimmed_msg) = Self::filter(&raw_message);
            let mut color_rng = thread_rng();
            println!("\x1B[{}m{}\x1B[0m: {}", color_rng.gen_range(31..36),username, trimmed_msg);
        }
    }
}

