use serde::Serialize;
use serde_json::Result;

pub type Integer = i32;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Params {
    Integer(Integer),
    String(String),
}

#[derive(Serialize)]
pub struct Command {
    pub id: u16,
    pub method: String,
    pub params: Vec<Params>,
}

impl Command {
    pub fn new(id: u16, method: &str) -> Command {
        Command {
            id,
            method: String::from(method),
            params: Vec::new(),
        }
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}
