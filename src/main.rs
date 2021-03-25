extern crate clap;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use clap::{App as ClapApp, Arg};

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
    let matches = ClapApp::new("moco-rs")
        .version("0.0.1")
        .author("ZhouYu <zhouyu.gg@qq.com>")
        .about("API模拟")
        .arg(Arg::new("json")
            .short('j')
            .long("json")
            .value_name("JSON")
            .about("指定加载的API列表文件")
            .takes_value(true))
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .about("指定服务端口号")
            .takes_value(true))
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .about("指定日志等级"))
        .subcommand(ClapApp::new("test")
            .about("controls testing features")
            .version("0.0.1")
            .author("")
            .arg(Arg::new("debug")
                .short('d')
                .about("")))
        .get_matches();

    let mut json_path = "test.json";
    let mut port = 8000;

    if let Some(v) = matches.value_of("json") {
        println!("Value for json: {}", v);
        json_path = v;
    }
    if let Some(v) = matches.value_of("port") {
        port = v.parse().unwrap();
    }


    println!("App Listening on port: {}", port);

    HttpServer::new(|| {
        App::new()
            // .service(echo)
            .service(web::resource("/**").route(web::route().to(echo)))
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}

