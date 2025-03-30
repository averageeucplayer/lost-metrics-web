use std::time::Duration;

use semver::VersionReq;
use serde::{Deserialize, Serialize};

// TO-DO lost-metrics-core-web?

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Encounter {
    pub id: String,
    pub updated_on: String,
    pub total_damage: i64,
    pub participants: Vec<Participant>
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Participant {
    pub id: u64,
    pub name: String,
    pub class: String
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SnifferSettings {
    pub process_name: String,
    pub port: u16,
    #[serde(with = "humantime_serde")]
    pub timeout: Duration
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SaveSettings {
    pub settings: Settings
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub version: VersionReq,
    pub sniffer: SnifferSettings
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoadResult {
    pub app_name: String,
    pub version: String,
    pub settings: Settings
}