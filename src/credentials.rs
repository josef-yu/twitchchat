use serde::{Deserialize, Serialize};
use std::path::Path;

static CREDENTIALS_PATH: &str = "./config";
static CREDENTIALS_FPATH: &str = "./config/credentials.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub auth: String,
    pub client_id: String
}

impl Credentials {
    pub fn get_credentials() -> Result<Credentials, &'static str> {
        let mut data = String::from(r#"{"auth": "", "client_id": ""}"#);

        if Path::new(CREDENTIALS_FPATH).exists() {
            data = std::fs::read_to_string(CREDENTIALS_FPATH)
                .expect("Unable to read file");
        }

        let json_data: Credentials = serde_json::from_str(&data)
            .expect("Unable to serialize data");

        Ok(json_data)
    }

    pub fn save_credentials(&self) -> Result<(), &'static str> {
        let json_data = serde_json::to_string(&self)
            .expect("Unable to deserialize");

        if !Path::new(CREDENTIALS_FPATH).exists() {
            std::fs::create_dir_all(CREDENTIALS_PATH)
                .expect("Unable to create path");
        }

        std::fs::write(CREDENTIALS_FPATH, json_data)
            .expect("Unable to write file");

        println!("Save successful");
        
        Ok(())
    }

    pub fn check_credentials(&self) -> Result<(), &'static str> {
        if self.auth.is_empty() {
            return Err("Token not found! Please enter a token")
        }

        if self.client_id.is_empty() {
            return Err("Client ID not found! Pleaser enter a Client ID")
        }

        Ok(())
    }
}