mod routes;
mod lib;

use std::env;
use std::net::SocketAddr;
use mongodb::{bson::doc, options::ClientOptions, Client};
use crate::routes as server_routes;
use crate::lib::constants;

#[tokio::main]
async fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(constants::SERVER_URL));
    let socket_address: SocketAddr = addr.parse().expect("valid socket Address");

    let db_client = connect_to_db().await.expect("Could not connect to database");

    let server = warp::serve(server_routes::routes_api(db_client)).try_bind(socket_address);
    println!("Running server at {}!", addr);
    server.await
}

async fn connect_to_db() -> mongodb::error::Result<Client> {
    let addr = env::var("MONGO_URL")
        .unwrap_or_else(|_err| String::from(constants::DB_URL));
    let mut client_options =
        ClientOptions::parse(addr)
            .await.expect("Failed to parse db string");
    client_options.app_name= Some("real_state_game_api".to_string());
    let client = Client::with_options(client_options)?;

    // Ping the server
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    Ok(client)
}
