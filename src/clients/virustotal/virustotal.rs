use crate::clients::{base, virustotal::IpAddress, virustotal::IpComments};
use reqwest;

const BASE_URL: &str = "https://www.virustotal.com/api/v3";

/// VirusTotal API object
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Client {
        Client { api_key }
    }

    pub async fn get_ip_whois(&self, ip: &str) -> Result<IpAddress, reqwest::StatusCode> {
        let url = format!("{}/ip_addresses/{}", BASE_URL, ip);

        let res: Result<IpAddress, reqwest::StatusCode> =
            base::get(url, Some(("x-apikey", &self.api_key))).await;

        res
    }

    pub async fn get_ip_comments(&self, ip: &str) -> Result<IpComments, reqwest::StatusCode> {
        let url = format!("{}/ip_addresses/{}/comments", BASE_URL, ip);

        let res: Result<IpComments, reqwest::StatusCode> =
            base::get(url, Some(("x-apikey", &self.api_key))).await;

        res
    }
}
