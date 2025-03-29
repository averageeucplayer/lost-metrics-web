use serde::Deserialize;

// TO-DO lost-metrics-core-web?

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Encounter {
    pub id: String,
    pub updated_on: String,
    pub total_damage: i64
}

#[derive(Debug, Default, Clone, Deserialize, Eq, PartialEq)]
pub struct Settings {

}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LoadResult {
    pub version: String,
    pub settings: Settings
}