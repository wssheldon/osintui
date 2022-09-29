use crate::app::App;
use crate::clients::{shodan, virustotal};
use crate::config::Config;
use anyhow::anyhow;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum IoEvent {
    VirusTotal(String),
    Shodan(String),
}

#[derive(Clone)]
pub struct Network<'a> {
    pub vt_client: virustotal::Client,
    pub shodan_client: shodan::Client,
    pub client_config: Config,
    pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(
        vt_client: virustotal::Client,
        shodan_client: shodan::Client,
        client_config: Config,
        app: &'a Arc<Mutex<App>>,
    ) -> Self {
        Network {
            vt_client,
            shodan_client,
            client_config,
            app,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::VirusTotal(query) => {
                self.virustotal_get_ip_whois(query).await;
            }
            IoEvent::Shodan(query) => {
                self.shodan_search_ip(query).await;
            }
        };

        let mut app = self.app.lock().await;
        app.is_loading = false;
    }

    async fn handle_error(&mut self, e: anyhow::Error) {
        let mut app = self.app.lock().await;
        app.handle_error(e);
    }

    async fn virustotal_get_ip_whois(&mut self, ip: String) {
        match self.vt_client.get_ip_whois(ip.as_str()).await {
            Ok(resp) => {
                let mut app = self.app.lock().await;
                app.virustotal.ip_whois_items = resp;
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn shodan_search_ip(&mut self, ip: String) {
        match self.shodan_client.search_ip(ip.as_str()).await {
            Ok(resp) => {
                let mut app = self.app.lock().await;
                app.shodan.search_ip_items = resp;
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }
}
