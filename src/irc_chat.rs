use std::net::TcpStream;
use std::io::ErrorKind;
use std::time::Duration;
use crossterm::event::{read, KeyEvent, Event, KeyCode, KeyModifiers, poll};
use chrono::{ Utc, TimeZone, Local, DateTime};
use crate::twitch::TwitchApiHandler;
use crate::credentials::Credentials;
use crate::logger::Logger;
use crate::codec::Codec;
use crate::message::Message;


pub struct IrcChatScraper<'a> {
    socket: Codec<TcpStream>,
    reconnect_time: u64,
    auth: &'a str,
    channel: &'a str,
    api: TwitchApiHandler,
    logger: Option<Logger>,
}
impl<'a> IrcChatScraper<'a> {
    const IRC_SERVER: &'a str = "irc.chat.twitch.tv:6667";

    pub fn connect(credentials: &'a Credentials, log_filename: Option<&'a str>, should_log: bool, channel: &'a str) -> std::io::Result<Self> {
        print!("Connecting...");
        if let Ok(stream) = TcpStream::connect(Self::IRC_SERVER) {
            println!(" Success!");
            stream.set_read_timeout(Some(Duration::from_secs(180)))
                .expect("Setting read timeout failed!");

            let logger: Option<Logger> = if should_log {
                Some(Logger::init(log_filename, channel)?)
            } else {
                None
            };

            Ok(IrcChatScraper {
                socket: Codec::new(stream)?,
                reconnect_time: 0,
                auth: credentials.auth.as_str(),
                api: TwitchApiHandler::set(credentials.client_id.clone(),
                                           credentials.auth.clone(),
                                           String::from(channel.to_lowercase())),
                channel,
                logger
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
            self.socket = Codec::new(stream)?;
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

    fn continue_prompt(is_live: bool) -> std::io::Result<bool> {
        if !is_live {
            println!("This channel is offline. Do you want to continue? y/n");
            loop {
                match read().unwrap() {
                    Event::Key(KeyEvent{
                                   code: KeyCode::Char('y'),
                                   modifiers: KeyModifiers::NONE,
                               }) => return Ok(true),
                    Event::Key(KeyEvent{
                                   code: KeyCode::Char('n'),
                                   modifiers: KeyModifiers::NONE,
                               }) => return Err(std::io::Error::new(ErrorKind::Other, "Scraping aborted")),
                    _ => {}
                }
            }
        } else {
            Ok(true)
        }
    }

    fn check_key_press() -> std::io::Result<bool> {

        if poll(Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                               code: KeyCode::Char('q'),
                               modifiers: KeyModifiers::NONE,
                           }) => return Ok(true),
                _ => {}
            }
        }

        Ok(false)
    }

    fn init_irc(&mut self) -> std::io::Result<()> {
        let (_, is_live) = self.api.get_user_channel()
            .expect("Failed to fetch stream info.");
        Self::continue_prompt(is_live)?;

        self.socket.send("CAP REQ :twitch.tv/tags\n")?;
        self.socket.send(&*format!("PASS oauth:{}\n", self.auth))?;
        self.socket.send("NICK scraper\n")?;
        
        self.socket.send(&*format!("JOIN #{}\n", self.channel.to_lowercase()))?;

        /*Receives the first 11 lines of messages from the IRC server
        so that when scrape() is called, the messages to be received are
        the twitch chat messages.
         */
        for _ in 0..11 {
            let msg = self.socket.receive()?;
            if msg.contains("NOTICE * :Login unsuccessful") {
                return Err(std::io::Error::new(ErrorKind::Other, "Invalid token"));
            } else if msg.contains("Bad Request") {
                return Err(std::io::Error::new(ErrorKind::Other, "Bad Request"));
            }
            if msg.is_empty() {
                self.reconnect()?;
                break;
            }
        }

        println!("Now {} twitch.tv/{} chat", if self.logger.is_some() {"scraping"} else {"watching"} ,self.channel);
        Ok(())
    }

    pub fn scrape(&mut self) -> std::io::Result<()> {
        self.init_irc()?;

        loop {
            if Self::check_key_press()? {
                break
            }

            let raw_message = self.socket.receive()?;

            if raw_message.is_empty() {
                self.reconnect()?;
                continue
            }

            if raw_message.starts_with("PING") {
                self.socket.send("PONG\n")?;
                continue
            }

            let message = Message::filter(&raw_message);
            message.print();

            if let Some(logger) = &mut self.logger {
                let stamp_date = Utc.timestamp_millis(message.timestamp as i64);
                let converted: DateTime<Local> = DateTime::from(stamp_date);

                logger.write(format!("[{}]{}: {}", converted.time(), message.username, message.body))?;
            }
        }

        Ok(())
    }
}