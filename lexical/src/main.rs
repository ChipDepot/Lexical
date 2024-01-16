mod application;
mod component;
mod location;
mod parsing;
mod utils;

#[macro_use]
extern crate log;

use std::panic::catch_unwind;
use std::path::PathBuf;
use std::process::exit;

use crate::utils::file_handler;
use crate::utils::requester;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Lexical started");

    let app = catch_unwind(|| parsing::parse_yaml().unwrap()).unwrap_or_else(|_| {
        error!(
            "Could not get valid application from file {:?}",
            file_handler::get_argument::<PathBuf>("-f").unwrap()
        );
        exit(-1);
    });
    // };

    info!("Application {} definition was validated!", &app.name);

    // println!("{}", serde_json::to_string_pretty(&app).unwrap());

    // Send context to Bran instance
    requester::send_context(&app).await.unwrap();
}
