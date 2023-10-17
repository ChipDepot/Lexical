mod application;
mod component;
mod location;
mod parser;
mod utils;

#[macro_use]
extern crate log;

use crate::parser::parser::Parser;
use crate::utils::requester;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Lexical started");
    let app = Parser::parse_yaml().unwrap();

    requester::send_context(&app).await.unwrap();

    println!("{}", serde_json::to_string_pretty(&app).unwrap());
}
