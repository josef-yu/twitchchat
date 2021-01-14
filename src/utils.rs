use std::net::TcpStream;
use std::io::prelude::*;

pub struct Codec {
    reader: std::io::BufReader<TcpStream>,
    writer: std::io::LineWriter<TcpStream>
}

impl Codec {
    pub fn new(stream: TcpStream) -> std::io::Result<Self> {
        let writer = std::io::LineWriter::new(stream.try_clone()?);
        let reader = std::io::BufReader::new(stream);
        Ok(Self{reader,writer})
    }

    pub fn send(&mut self, message: &str) -> std::io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        Ok(())
    }

    pub fn receive(&mut self) -> std::io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
}




