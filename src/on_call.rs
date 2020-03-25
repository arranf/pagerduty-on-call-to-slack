use chrono::{DateTime, Utc};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OnCallResponse {
    // TODO Change to oncalls by serde
    #[serde(rename = "oncalls")]
    pub on_calls: Vec<OnCall>,
    limit: usize,
    offset: usize,
    more: bool,
    total: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct OnCall {
    pub user: User,
    pub schedule: Schedule,
    pub escalation_level: usize,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename = "summary")]
    pub name: String,
    html_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub summary: String,
    html_url: String,
}
