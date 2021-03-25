use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Request {
    pub(crate) method: String,
    pub(crate) uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(default)]
    pub(crate) text: Option<String>,
    #[serde(default)]
    pub(crate) json: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct Api {
    description: String,
    pub(crate) request: Request,
    pub(crate) response: Response,
}

pub fn read_api_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Api>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let apis = serde_json::from_reader(reader)?;

    Ok(apis)
}

fn _load_map() {
    let api_arr = read_api_from_file("./test.json").unwrap();
    let mut api_map = HashMap::new();
    for api in &api_arr {
        let key = api.request.method.to_string() + &":".to_string() + &api.request.uri.to_string();
        println!("add {} to map", key);
        api_map.insert(key, api);
    }
}

#[cfg(test)]
mod tests {
    use crate::json_reader::{load_map, read_api_from_file};

    #[test]
    fn test_read_api_from_file() {
        let result = read_api_from_file("./test.json");
        println!("result: {:#?}", result);
    }

    #[test]
    fn test_load_map() {
        load_map()
    }
}

