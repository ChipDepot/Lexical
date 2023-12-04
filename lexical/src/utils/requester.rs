use reqwest::{Client, StatusCode};
use starduck::application::Application;
use url::Url;

use crate::utils::file_handler;

pub(crate) async fn send_context(app: &Application) -> Result<(), ()> {
    let app_name = file_handler::get_argument::<String>("-a").unwrap();
    let url = file_handler::get_argument::<Url>("-b")
        .unwrap()
        .join(&app_name)
        .unwrap();

    let client = Client::new();

    match client.post(url.clone()).json(app).send().await {
        Ok(k) => match k.status() {
            StatusCode::OK => (),
            _ => panic!("Recived {} from {}", k.status(), url.to_string()),
        },
        Err(_) => panic!("Could not reach {}. Is the app running?", url.to_string()),
    };

    info!(
        "Objective Architecture for {} has been POSTed to {}",
        app_name, url
    );

    return Ok(());
}
