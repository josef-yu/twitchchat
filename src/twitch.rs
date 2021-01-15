use reqwest::header::{ACCEPT};
use crate::CLIENT_ID;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Blob {
    #[serde(rename = "_total")]
    total: u64,
    pub channels: Vec<TwitchUser>,
}

#[derive(Deserialize, Debug)]
pub struct TwitchUser {
    #[serde(rename = "_id")]
    pub id: u64,
    pub name: String,
    pub status: Option<String>,
    #[serde(flatten)]
    pub rest: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct TwitchUserStream {
    pub stream: Option<StreamInfo>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct StreamInfo {
    #[serde(flatten)]
    pub rest: serde_json::Value,
}

pub struct TwitchApiHandler;
impl TwitchApiHandler {
    fn get_user_id(channel: &str) -> reqwest::Result<u64> {
        let client = reqwest::blocking::Client::new();
        let blob: Blob = client.get(
            "https://api.twitch.tv/kraken/search/channels")
            .query(&[("query", channel)])
            .header(ACCEPT, "application/vnd.twitchtv.v5+json")
            .header("Client-ID", CLIENT_ID)
            .send()?
            .json()?;
        let mut id = 0;
        for ch in blob.channels {
            if ch.name == channel {
                id = ch.id;
            }
        }

        Ok(id)
    }

    pub fn is_live(channel: &str) -> reqwest::Result<bool> {
        let id = Self::get_user_id(channel)?;
        let client = reqwest::blocking::Client::new();
        let blob: TwitchUserStream = client.get(
            &*format!("https://api.twitch.tv/kraken/streams/{}", id))
            .header(ACCEPT, "application/vnd.twitchtv.v5+json")
            .header("Client-ID", CLIENT_ID)
            .send()?
            .json()?;
        if let Some(_) = blob.stream {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}