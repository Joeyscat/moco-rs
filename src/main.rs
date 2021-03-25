extern crate clap;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};

use crate::json_reader::read_api_from_file;

mod json_reader;


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
                    };
                }
            };
        }
    }
    HttpResponse::NotFound().body("Not Found")
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

