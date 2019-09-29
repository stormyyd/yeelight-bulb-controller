pub type Integer = i32;

pub enum Params {
    Integer(Integer),
    String(String),
}

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

    pub fn to_json(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|p| match p {
                Params::Integer(i) => i.to_string(),
                Params::String(s) => format!("\"{}\"", s),
            })
            .collect::<Vec<String>>()
            .join(",");
        format!(
            "{{\"id\":{},\"method\":\"{}\",\"params\":[{}]}}",
            self.id, &self.method, params
        )
    }
}
