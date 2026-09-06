use actix_files::Files;
use actix_web::{App, HttpServer};
use local_ip_address::list_afinet_netifas;
use std::env;
/*
 * cargo run -- -p 3000
 * app -p 8888
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port: u16 = args
        .iter()
        .position(|x| x == "-p")
        .and_then(|i| args.get(i + 1))
        .and_then(|x| x.parse().ok())
        .unwrap_or(8888);
    println!("______________________________________________");
    match list_afinet_netifas() {
        Ok(interfaces) => {
            for (name, ip) in interfaces {
                println!("{}: {}", name, ip);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("______________________________________________");
    println!("Directory: ./static");
    println!("PORT: {}", port);
    println!("______________________________________________");
    HttpServer::new(|| App::new().service(Files::new("/", "./static").index_file("index.html")))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
