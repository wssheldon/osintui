use crate::clients::{base, censys::CensysSearchIp};
use reqwest;

const BASE_URL: &str = "https://search.censys.io/api/v2";

#[derive(Clone)]
pub struct Client {
    api_id: String,
    api_key: String,
}

impl Client {
    pub fn new(api_id: String, api_key: String) -> Client {
        Client { api_id, api_key }
    }

    pub async fn search_ip(&self, ip: &str) -> Result<CensysSearchIp, reqwest::StatusCode> {
        let url = format!("{}/hosts/{}", BASE_URL, ip);
        let res: Result<CensysSearchIp, reqwest::StatusCode> =
            base::get(url, None, Some((&self.api_id, &self.api_key))).await;

        res
    }
}
