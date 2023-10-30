
/****************** IMPORTS ******************/
use reqwest::Error;
use reqwest::header::USER_AGENT;
// mod objects;   import local file
use serde::{Deserialize,Serialize};
use structured_logger::{async_json::new_writer, Builder, unix_ms};
use log::{info, error, warn};
use tokio::{io, time, fs::File};
// use std::{fs::File, io::stdout};



/****************** MAIN ******************/
#[tokio::main]
async fn main() -> Result<(), Error> {
    let log_file = File::options()
    .create(true)
    .append(true)
    .open("data/app.log")
    .await
    .unwrap();

    Builder::new()
        // Optional: set a specific async writer (format to JSON, write to stdout) for target "api".
        .with_target_writer("api", new_writer(io::stdout()))
        .with_target_writer("*", new_writer(log_file))
        .init();


    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);




    info!("Setting up client request");
    let client: reqwest::Client = reqwest::Client::new();
    let my_request = client.get(request_url).header(USER_AGENT, "My Rust Program 1.0");
    let future_response = my_request.send();

    let resolved_response = future_response.await.unwrap();
    error!("Test");


    // // This works but consumes response
    // let thing = resolved_response.text().await.unwrap(); // why do I need to await and unwrap again?
    // println!("{}", thing);

    // This doesn't work because Response got consumed
    // How do I call both text() and json() off of one response?
    // Json is actually returned as a list, to deserialize we lust something like list<User>
    warn!("Note: Running json() or text() consumes the Response object");
    let mything = resolved_response.json::<Vec<User>>().await.unwrap();
    println!("{}", mything[0].login);

    log::info!(target: "api",
        method = "GET",
        path = "/hello",
        status = 200_u16,
        start = unix_ms(),
        elapsed = 10_u64,
        something = "hello",
        kv = log::as_serde!(mything[0]);
        "whatever",
);

    time::sleep(time::Duration::from_secs(1)).await;

    Ok(())
}


/****************** STRUCT/TRAITS ******************/
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub login: String,
    pub id: u32,
}