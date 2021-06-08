use std::error::Error;

use isahc::{AsyncBody, HttpClient, Response};
use serde_json::to_string;

use crate::log::VioletLog;

pub struct VioletMail {
    base_url: String,
    client: HttpClient
}

type VioletError = Box<dyn Error + Send + Sync + 'static>;

impl VioletMail {
    pub fn new(identifier: u32, token: String) -> Result<Self, VioletError> {
        let client = HttpClient::builder()
            .default_header("Authorization", token)
            .default_header("Content-Type", "application/json")
            .build()?;

        Ok(Self {
            base_url: format!("https://violet.zuraaa.com/api/apps/{}/events", identifier),
            client
        })
    }

    pub async fn send_log(&self, violet_log: VioletLog) -> Result<Response<AsyncBody>, VioletError> {
        let body = to_string(&violet_log)?;

        Ok(self.client.post_async(&self.base_url, body).await?)
    }
}
