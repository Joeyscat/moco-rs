use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Request {
    method: String,
    uri: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    json: Option<Value>,
}

#[derive(Deserialize, Debug)]
struct Api {
    description: String,
    request: Request,
    response: Response,
}

fn read_api_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Api>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let apis = serde_json::from_reader(reader)?;

    Ok(apis)
}

// #[get("/**")]
async fn echo(req: HttpRequest) -> HttpResponse {
    let req_key = req.method().to_string() + &":".to_string() + req.uri().path();

    let api_arr = read_api_from_file("./test.json").unwrap();
    for api in api_arr {
        let key =
            api.request.method.to_uppercase() + &":".to_string() + &api.request.uri.to_string();
        if key == req_key {
            return match &api.response.text {
                Some(text) => HttpResponse::Ok().body(text),
                None => {
                    return match api.response.json {
                        Some(json) => {
                            let j = serde_json::to_string(&json);
                            return match j {
                                Ok(s) => HttpResponse::Ok().body(s.clone()),
                                Err(e) => HttpResponse::Ok().body(e.to_string()),
                            };
                        }
                        None => HttpResponse::Ok().body(">_<"),
                    }
                }
            };
        }
    }
    HttpResponse::NotFound().body("Not Found")
}

fn load_map() {
    let api_arr = read_api_from_file("./test.json").unwrap();
    let mut api_map = HashMap::new();
    for api in &api_arr {
        let key = api.request.method.to_string() + &":".to_string() + &api.request.uri.to_string();
        println!("add {} to map", key);
        api_map.insert(key, api);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(echo)
            .service(web::resource("/**").route(web::route().to(echo)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;

    use serde_json::value::Value::{Array, Object};
    use serde_json::Value;

    use crate::Api;

    fn a() -> Result<Vec<Api>, Box<Error>> {
        let file = File::open("test.json")?;
        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }

    #[test]
    fn test_a() {
        let result = a();
        println!("result: {:#?}", result);
    }

    fn read_json() -> Result<Value, Box<Error>> {
        let file = File::open("test.json")?;
        let reader = BufReader::new(file);

        let v: Value = serde_json::from_reader(reader)?;
        Ok(v)
    }

    #[test]
    fn test_read_json() {
        let result = read_json();
        println!("result: {:#?}", result);
    }
}
