use crate::clients::{base, shodan::ShodanSearchIp};
use reqwest;

const BASE_URL: &str = "https://api.shodan.io";

#[derive(Clone)]
pub struct Client {
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Client {
        Client { api_key }
    }

    pub async fn search_ip(&self, ip: &str) -> Result<ShodanSearchIp, reqwest::StatusCode> {
        let url = format!("{}/shodan/host/{}?key={}", BASE_URL, ip, self.api_key);
        let res: Result<ShodanSearchIp, reqwest::StatusCode> = base::get(url, None, None).await;

        res
    }
}
