use regex::Regex;
use gh_emoji::Replacer;
use rand::Rng;

pub struct Message {
    pub username: String,
    pub body: String,
    pub username_color: String,
    pub timestamp: u64
}

impl Message {
    pub fn filter(raw_message: &str) -> Message {
        let cap = Regex::new(
            r".*;color=(.*?);display-name=(.*?);.*;tmi-sent-ts=(.*?);.*:(.*)!.*@.*\.tmi\.twitch\.tv PRIVMSG #.* :(.*)")
            .unwrap()
            .captures(raw_message).unwrap();
        let demojied_msg = Replacer::new().replace_all(&cap[5]);
        let username = if (&cap[2]).is_empty() {
            &cap[4]
        } else {
            &cap[2]
        };
        let color = &cap[1];
        let timestamp: u64 = (&cap[3]).parse::<u64>().unwrap();

        Message {
            body: demojied_msg.to_string(),
            username: username.to_string(),
            username_color: color.to_string(),
            timestamp
        }
    }

    pub fn print(&self) {

        if self.username_color.is_empty() {
            let color = rand::thread_rng().gen_range(31..36);
            println!("\x1B[{}m{}\x1B[0m: {}", color,self.username, self.body);
        } else {
            let rgb = raster::Color::hex(&self.username_color).unwrap();
            let colored_username = ansi_term::Color::RGB(rgb.r, rgb.g, rgb.b).paint(&self.username);
            println!("{}: {}", colored_username, self.body);
        };
    }
}