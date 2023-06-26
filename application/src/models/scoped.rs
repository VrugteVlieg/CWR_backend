use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct ScopedState {
    pub scope_name: String
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Animal {
    pub id: u16,
    pub name: String,
    pub species: String,
    pub sex: String,
    pub arks_no: String,
    pub dob: String,
    pub weight: u32,
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]pub struct DataRecording {
    pub weight: Vec<u32>,
    pub feces: Vec<bool>,
    pub gram: Vec<u32>,
    pub parasites: Vec<String>,
    pub comments: Vec<String>
}