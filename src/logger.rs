use crate::codec::{Codec};
use std::fs::File;
use chrono::Local;

pub struct Logger {
    codec: Codec<File>,
}

impl Logger {
    pub fn init(arg: Option<&str>,channel: &str) -> std::io::Result<Logger> {
        let filename = match arg {
            None => {
                let date  = Local::now().date().format("%Y-%m-%d").to_string();
                format!("./{}-{}.txt", channel, date)
            },
            Some(x) => format!("./{}", x)
        };

        let file = std::fs::OpenOptions::new()
            .write(true).read(true).append(true).create(true).open(filename)?;

        Ok(Logger {
            codec: Codec::new(file)?,
        })

    }

    pub fn write(&mut self, msg: String) -> std::io::Result<()> {
        let data = format!("{}\n", msg);
        self.codec.send(data.as_str())?;
        Ok(())
    }
}