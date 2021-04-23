use reqwest::header::{ACCEPT};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Blob {
    #[serde(rename = "_total")]
    total: u64,
    channels: Vec<TwitchUser>,
}

#[derive(Deserialize, Debug)]
struct TwitchUser {
    #[serde(rename = "_id")]
    id: u64,
    name: String,
    status: Option<String>,
    #[serde(flatten)]
    rest: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct TwitchUserStream {
    pub stream: Option<StreamInfo>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct StreamInfo {
    #[serde(flatten)]
    rest: serde_json::Value,
}

pub struct TwitchApiHandler{
    pub client_id: String,
    channel: String,
}

impl TwitchApiHandler {

    pub fn set(client_id: String, channel: String) -> TwitchApiHandler {
        TwitchApiHandler {
            client_id,
            channel
        }
    }

    fn get_user_id(&self) -> reqwest::Result<u64> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(
            "https://api.twitch.tv/kraken/search/channels")
            .query(&[("query", self.channel.clone())])
            .header(ACCEPT, "application/vnd.twitchtv.v5+json")
            .header("Client-ID", self.client_id.clone())
            .send()?
            .error_for_status()
            .expect("Invalid Client ID!");

        let blob: Blob = response.json()?;

        let mut id = 0;
        for ch in blob.channels {
            if ch.name == self.channel {
                id = ch.id;
            }
        }

        Ok(id)
    }

    pub fn is_live(&self) -> reqwest::Result<bool> {
        let id = self.get_user_id()?;
        let client = reqwest::blocking::Client::new();
        let response = client.get(
            &*format!("https://api.twitch.tv/kraken/streams/{}", id))
            .header(ACCEPT, "application/vnd.twitchtv.v5+json")
            .header("Client-ID", self.client_id.clone())
            .send()?
            .error_for_status()
            .expect("Invalid Client ID!");

        let blob: TwitchUserStream = response.json()?;

        if let Some(_) = blob.stream {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}