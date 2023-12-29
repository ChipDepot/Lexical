mod application;
mod component;
mod location;
mod parsing;
mod utils;

#[macro_use]
extern crate log;

use crate::utils::requester;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Lexical started");

    let app = parsing::parse_yaml().unwrap();

    // Send context to Bran instance
    requester::send_context(&app).await.unwrap();

    println!("{}", serde_json::to_string_pretty(&app).unwrap());
}
