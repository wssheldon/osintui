use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CensysSearchIp {
    pub code: i32,
    pub status: String,
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub ip: String,
    pub location_updated_at: Option<String>,
    pub autonomous_system_updated_at: String,
    pub last_updated_at: Option<String>,
    pub services: Vec<Services>,
    pub location: Location,
    pub autonomous_system: AutonomousSystem,
    pub operating_sytem: Option<OperatingSystem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Services {
    pub port: Option<i32>,
    pub service_name: Option<String>,
    pub transport_protocol: Option<String>,
    pub extended_service_name: Option<String>,
    pub certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub continent: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub postal_code: Option<String>,
    pub timezone: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub registered_country: Option<String>,
    pub registered_country_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutonomousSystem {
    pub asn: Option<i32>,
    pub description: Option<String>,
    pub bgp_prefix: Option<String>,
    pub name: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperatingSystem {
    pub product: String,
    pub vendor: Option<String>,
    pub version: Option<String>,
    pub edition: Option<String>,
    pub uniform_resource_identifier: Option<String>,
}

impl CensysSearchIp {
    pub fn summary_to_vec(&self) -> Vec<Vec<String>> {
        vec![
            vec!["IPv4".to_owned(), self.result.ip.to_string()],
            vec![
                "Network".to_owned(),
                self.result
                    .autonomous_system
                    .name
                    .as_deref()
                    .unwrap_or("N/A")
                    .to_owned(),
            ],
            vec![
                "ASN".to_owned(),
                self.result
                    .autonomous_system
                    .asn
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| "N/A".to_owned()),
            ],
            vec![
                "Routing".to_owned(),
                self.result
                    .autonomous_system
                    .bgp_prefix
                    .as_deref()
                    .unwrap_or("N/A")
                    .to_owned(),
            ],
            vec![
                "Operating System".to_owned(),
                self.result
                    .operating_sytem
                    .as_ref()
                    .map_or_else(|| "N/A".to_owned(), |os| os.product.to_string()),
            ],
        ]
    }
}
