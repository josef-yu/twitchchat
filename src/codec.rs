use std::net::TcpStream;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::fs::File;

pub struct Codec<T>
    where T: Read + Write {
    reader: BufReader<T>,
    writer: LineWriter<T>
}

impl<T> Codec<T>
where T: TryClone<T> + Read + Write {
    pub fn new(stream: T) -> std::io::Result<Self> {
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self{reader, writer})
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

pub trait TryClone<T> {
    fn try_clone(&self) -> std::io::Result<T>;
}

impl TryClone<TcpStream> for TcpStream {
    fn try_clone(&self) -> std::io::Result<TcpStream> {
        Ok(self.try_clone()?)
    }
}

impl TryClone<File> for File {
    fn try_clone(&self) -> std::io::Result<File> {
        Ok(self.try_clone()?)
    }
}

