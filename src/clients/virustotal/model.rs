use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpAddress {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub attributes: Attributes,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub as_owner: String,
    pub whois: String,
    pub asn: i32,
    pub continent: String,
    pub network: String,
    pub total_votes: Votes,
    pub last_analysis_results: HashMap<String, AnalysisResult>,
    pub last_analysis_stats: AnalysisStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResult {
    #[serde(rename = "engine_name")]
    pub engine_name: String,
    pub result: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Votes {
    pub harmless: i32,
    pub malicious: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisStats {
    pub harmless: i32,
    pub malicious: i32,
    pub suspicious: i32,
    pub timeout: i32,
    pub undetected: i32,
}
