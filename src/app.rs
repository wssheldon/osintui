use super::config::Config;
use super::user_config::UserConfig;
use crate::clients::censys::{
    AutonomousSystem, CensysSearchIp, Coordinates, Location as CensysLocation, OperatingSystem,
    Result, Services,
};
use crate::clients::shodan::{Location as ShodanLocation, ServiceData, ShodanSearchIp};
use crate::clients::virustotal::{
    AnalysisStats, CommentAttributes, CommentAuthor, CommentVotes, IpAddress, IpAttributes,
    IpCommentAttributes, IpCommentData, IpComments, IpData, Votes,
};
use crate::network::IoEvent;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use tui::layout::Rect;

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Empty,
    hovered_block: ActiveBlock::Home,
};

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Search,
    SearchResult,
    Censys,
    CensysGeoLookup,
    Shodan,
    ShodanGeoLookup,
    VirustotalDetection,
    VirustotalDetails,
    VirustotalCommunity,
    Unloaded,
    NotQueried,
    NotFound,
    Error,
}

pub const VIRUSTOTAL_MENU: [&str; 3] = ["Detection", "Details", "Community"];

pub struct Virustotal {
    pub status: ResultStatus,
    pub selected_index: usize,
    pub analysis_result_index: usize,
    pub whois_result_index: usize,
    pub scan_table: ScanTable,
    pub ip_whois_items: IpAddress,
    pub ip_comment_items: IpComments,
    pub comment_authors: CommentAuthor,
    pub comment_scroll: u16,
}

pub const SHODAN_MENU: [&str; 2] = ["General", "Geo-Lookup"];

pub struct Shodan {
    pub status: ResultStatus,
    pub service_index: usize,
    pub menu_index: usize,
    pub search_ip_items: ShodanSearchIp,
}

pub const CENSYS_MENU: [&str; 2] = ["Summary", "Geo-Lookup"];

pub struct Censys {
    pub status: ResultStatus,
    pub service_index: usize,
    pub menu_index: usize,
    pub search_ip_items: CensysSearchIp,
}

pub struct ScanTable {
    pub selected_index: usize,
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Error,
    Empty,
    SearchResult,
    Home,
    Input,
    CensysMenu,
    CensysServices,
    CensysNotFound,
    CensysNotQueried,
    CensysUnloaded,
    ShodanMenu,
    ShodanServices,
    ShodanNotFound,
    ShodanNotQueried,
    ShodanUnloaded,
    VirustotalMenu,
    VirustotalSummary,
    VirustotalResults,
    VirustotalWhois,
    VirustotalComments,
    VirustotalNotFound,
    VirustotalNotQueried,
    VirustotalUnloaded,
}

pub enum ResultStatus {
    NotQueried,
    NotFound,
    Found,
}

pub struct App {
    navigation_stack: Vec<Route>,
    pub user_config: UserConfig,
    pub client_config: Config,
    pub home_scroll: u16,
    pub is_loading: bool,
    pub is_input_error: bool,
    pub api_error: String,
    pub size: Rect,
    // Inputs:
    // input is the string for input;
    // input_idx is the index of the cursor in terms of character;
    // input_cursor_position is the sum of the width of characters preceding the cursor.
    // Reason for this complication is due to non-ASCII characters, they may
    // take more than 1 bytes to store and more than 1 character width to display.
    pub input: Vec<char>,
    pub input_idx: usize,
    pub input_cursor_position: u16,
    pub censys: Censys,
    pub shodan: Shodan,
    pub virustotal: Virustotal,
    io_tx: Option<Sender<IoEvent>>,
}

impl Default for App {
    fn default() -> Self {
        App {
            api_error: String::new(),
            virustotal: Virustotal {
                status: ResultStatus::NotQueried,
                selected_index: 0,
                analysis_result_index: 0,
                whois_result_index: 0,
                comment_scroll: 0,
                scan_table: ScanTable { selected_index: 0 },
                ip_whois_items: IpAddress {
                    data: IpData {
                        attributes: IpAttributes {
                            as_owner: String::new(),
                            asn: 0,
                            continent: String::new(),
                            network: String::new(),
                            whois: Some(String::new()),
                            total_votes: Votes {
                                harmless: 0,
                                malicious: 0,
                            },
                            last_analysis_results: HashMap::new(),
                            last_analysis_stats: AnalysisStats {
                                harmless: 0,
                                malicious: 0,
                                suspicious: 0,
                                timeout: 0,
                                undetected: 0,
                            },
                        },
                        id: String::new(),
                    },
                },
                ip_comment_items: IpComments {
                    data: vec![IpCommentData {
                        id: String::new(),
                        attributes: IpCommentAttributes {
                            date: 0,
                            html: String::new(),
                            text: String::new(),
                            votes: CommentVotes {
                                abuse: 0,
                                negative: 0,
                                positive: 0,
                            },
                        },
                    }],
                },
                comment_authors: CommentAuthor {
                    data: CommentAttributes {
                        first_name: String::new(),
                        last_name: String::new(),
                        profile_phrase: String::new(),
                        status: String::new(),
                        user_since: 0,
                    },
                    id: String::new(),
                },
            },
            shodan: Shodan {
                status: ResultStatus::NotQueried,
                service_index: 0,
                menu_index: 0,
                search_ip_items: ShodanSearchIp {
                    ip_str: Some(String::new()),
                    org: String::new(),
                    isp: String::new(),
                    asn: String::new(),
                    os: Some(String::new()),
                    domains: Some(vec![String::new()]),
                    hostnames: Some(vec![String::new()]),
                    data: Some(vec![ServiceData {
                        service: Some(String::new()),
                        product: Some(String::new()),
                        transport: Some(String::new()),
                        port: 0,
                        location: Some(ShodanLocation {
                            area_code: Some(0),
                            city: Some(String::new()),
                            country_code: Some(String::new()),
                            country_name: Some(String::new()),
                            latitude: Some(0.0),
                            longitude: Some(0.0),
                            region_code: Some(String::new()),
                        }),
                    }]),
                    ports: Some(vec![0]),
                    latitude: 0.00,
                    longitude: 0.00,
                    city: Some(String::new()),
                    country_code: Some(String::new()),
                    country_name: Some(String::new()),
                },
            },
            censys: Censys {
                status: ResultStatus::NotQueried,
                service_index: 0,
                menu_index: 0,
                search_ip_items: CensysSearchIp {
                    code: 0,
                    status: String::new(),
                    result: Result {
                        ip: String::new(),
                        location_updated_at: Some(String::new()),
                        autonomous_system_updated_at: String::new(),
                        last_updated_at: Some(String::new()),
                        services: vec![Services {
                            port: Some(0),
                            service_name: Some(String::new()),
                            transport_protocol: Some(String::new()),
                            extended_service_name: Some(String::new()),
                            certificate: Some(String::new()),
                        }],
                        location: CensysLocation {
                            continent: Some(String::new()),
                            country: Some(String::new()),
                            country_code: Some(String::new()),
                            postal_code: Some(String::new()),
                            timezone: Some(String::new()),
                            coordinates: Some(Coordinates {
                                latitude: 0.000,
                                longitude: 0.000,
                            }),
                            registered_country: Some(String::new()),
                            registered_country_code: Some(String::new()),
                        },
                        autonomous_system: AutonomousSystem {
                            asn: Some(0),
                            description: Some(String::new()),
                            bgp_prefix: Some(String::new()),
                            name: Some(String::new()),
                            country_code: Some(String::new()),
                        },
                        operating_sytem: Some(OperatingSystem {
                            product: String::new(),
                            vendor: Some(String::new()),
                            version: Some(String::new()),
                            edition: Some(String::new()),
                            uniform_resource_identifier: Some(String::new()),
                        }),
                    },
                },
            },
            navigation_stack: vec![DEFAULT_ROUTE],
            input: vec![],
            input_idx: 0,
            is_loading: false,
            is_input_error: false,
            io_tx: None,
            home_scroll: 0,
            input_cursor_position: 0,
            user_config: UserConfig::new(),
            client_config: Config::new(),
            size: Rect::default(),
        }
    }
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>, user_config: UserConfig, client_config: Config) -> App {
        App {
            io_tx: Some(io_tx),
            user_config,
            client_config,
            ..App::default()
        }
    }

    // Send a network event to the network thread
    pub fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in network.rs
        self.is_loading = true;
        if let Some(io_tx) = &self.io_tx {
            if let Err(e) = io_tx.send(action) {
                self.is_loading = false;
                println!("Error from dispatch {}", e);
                // TODO: handle error
            };
        }
    }

    pub fn handle_error(&mut self, e: anyhow::Error) {
        self.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
        self.api_error = e.to_string();
    }

    pub fn push_navigation_stack(
        &mut self,
        next_route_id: RouteId,
        next_active_block: ActiveBlock,
    ) {
        if !self
            .navigation_stack
            .last()
            .map(|last_route| last_route.id == next_route_id)
            .unwrap_or(false)
        {
            self.navigation_stack.push(Route {
                id: next_route_id,
                active_block: next_active_block,
                hovered_block: next_active_block,
            });
        }
    }

    pub fn get_current_route(&self) -> &Route {
        // if for some reason there is no route return the default
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    ) {
        let current_route = self.get_current_route_mut();
        if let Some(active_block) = active_block {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered_block {
            current_route.hovered_block = hovered_block;
        }
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        if self.navigation_stack.len() == 1 {
            None
        } else {
            self.navigation_stack.pop()
        }
    }
}
