use crate::app::App;
use crate::clients::{censys, shodan, virustotal};
use crate::config::Config;
use anyhow::anyhow;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
#[allow(dead_code)]
pub enum IoEvent {
    Censys(String),
    VirusTotal(String),
    VirustotalComments(String),
    VirustotalCommentAuthor(String),
    Shodan(String),
}

#[derive(Clone)]
pub struct Network<'a> {
    pub censys_client: censys::Client,
    pub shodan_client: shodan::Client,
    pub vt_client: virustotal::Client,
    pub client_config: Config,
    pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(
        censys_client: censys::Client,
        shodan_client: shodan::Client,
        vt_client: virustotal::Client,
        client_config: Config,
        app: &'a Arc<Mutex<App>>,
    ) -> Self {
        Network {
            censys_client,
            shodan_client,
            vt_client,
            client_config,
            app,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::Censys(query) => {
                self.censys_search_ip(query).await;
            }
            IoEvent::Shodan(query) => {
                self.shodan_search_ip(query).await;
            }
            IoEvent::VirusTotal(query) => {
                self.virustotal_get_ip_whois(query).await;
            }
            IoEvent::VirustotalComments(query) => {
                self.virustotal_get_ip_comments(query.clone()).await;
            }
            IoEvent::VirustotalCommentAuthor(query) => {
                self.virustotal_get_comment_author(query.clone()).await;
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

    async fn virustotal_get_ip_comments(&mut self, ip: String) {
        match self.vt_client.get_ip_comments(ip.as_str()).await {
            Ok(resp) => {
                let mut app = self.app.lock().await;
                app.virustotal.ip_comment_items = resp;
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn virustotal_get_comment_author(&mut self, comment_id: String) {
        match self
            .vt_client
            .get_comment_author(&comment_id.as_str())
            .await
        {
            Ok(resp) => {
                let mut app = self.app.lock().await;
                app.virustotal.comment_authors = resp;
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

    async fn censys_search_ip(&mut self, ip: String) {
        match self.censys_client.search_ip(ip.as_str()).await {
            Ok(resp) => {
                let mut app = self.app.lock().await;
                app.censys.search_ip_items = resp;
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }
}
