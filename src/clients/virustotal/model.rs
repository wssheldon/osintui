use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    pub data: IpData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpData {
    pub attributes: IpAttributes,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpAttributes {
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
pub struct AnalysisResult {
    #[serde(rename = "engine_name")]
    pub engine_name: String,
    pub result: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpComments {
    pub data: Vec<IpCommentData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpCommentData {
    pub attributes: IpCommentAttributes,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpCommentAttributes {
    pub date: usize,
    pub html: String,
    pub text: String,
    pub votes: CommentVotes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentVotes {
    pub abuse: i32,
    pub negative: i32,
    pub positive: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentAuthor {
    pub data: CommentAttributes,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentAttributes {
    pub first_name: String,
    pub last_name: String,
    pub profile_phrase: String,
    pub status: String,
    pub user_since: usize,
}
