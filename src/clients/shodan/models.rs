use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShodanSearchIp {
    #[serde(rename = "ip_str")]
    pub ip_str: Option<String>,
    pub org: String,
    pub isp: String,
    pub asn: String,
    pub os: Option<String>,
    pub domains: Option<Vec<String>>,
    pub hostnames: Option<Vec<String>>,
    pub data: Option<Vec<ServiceData>>,
    pub ports: Option<Vec<i32>>,
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceData {
    #[serde(rename = "data")]
    pub service: Option<String>,
    pub product: Option<String>,
    pub port: i32,
    pub transport: Option<String>,
    pub location: Option<Location>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub area_code: Option<usize>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub region_code: Option<String>,
}

impl ShodanSearchIp {
    pub fn summary_to_vec(&self) -> Vec<Vec<String>> {
        vec![
            vec![
                String::from("IPv4"),
                self.ip_str.as_deref().unwrap_or("N/A").to_string(),
            ],
            vec![
                String::from("Domains"),
                self.domains
                    .as_ref()
                    .map_or(String::from("N/A"), |x| x.concat()),
            ],
            vec![
                String::from("City"),
                self.city.as_deref().unwrap_or("N/A").to_string(),
            ],
            vec![String::from("Organization"), self.org.to_string()],
            vec![String::from("ISP"), self.isp.to_string()],
            vec![String::from("ASN"), self.asn.to_string()],
            vec![
                String::from("Operating System"),
                self.os.as_deref().unwrap_or("N/A").to_string(),
            ],
        ]
    }
}
