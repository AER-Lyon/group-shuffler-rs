use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InputRecord {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "master", default)]
    pub master: String,
    #[serde(rename = "member0", default)]
    pub member0: String,
    #[serde(rename = "member1", default)]
    pub member1: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[serde(rename = "Login")]
    pub login: String,
}

pub enum FileType {
    TeamsList,  // Format "name;code;master;member0;member1"
    LoginsList, // Format avec au moins des logins
}
