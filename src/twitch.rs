use reqwest::header::{ACCEPT};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Blob<T> {
    data: Vec<T>,
    #[serde(flatten)]
    rest: serde_json::Value
}

#[derive(Deserialize, Debug)]
struct TwitchUser {
    id: String,
    broadcaster_login: String,
    is_live: bool,
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
    pub token: String,
    channel: String,
}

impl TwitchApiHandler {

    pub fn set(client_id: String, token: String, channel: String) -> TwitchApiHandler {
        TwitchApiHandler {
            client_id,
            channel,
            token
        }
    }

    pub fn get_user_channel(&self) -> reqwest::Result<(String, bool)> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(
            "https://api.twitch.tv/helix/search/channels")
            .query(&[("query", self.channel.clone())])
            .header(ACCEPT, "application/vnd.twitchtv.v5+json")
            .header("Client-ID", self.client_id.clone())
            .header("Authorization", format!("Bearer {}", self.token.clone()))
            .send()?
            .error_for_status()
            .expect("Invalid Client ID!");

        let blob: Blob<TwitchUser> = response.json()?;

        let mut id = String::from("0");
        let mut is_live = false;
        for ch in blob.data {
            if ch.broadcaster_login == self.channel {
                id = ch.id;
                is_live = ch.is_live;
            }
        }

        Ok((id, is_live))
    }
}