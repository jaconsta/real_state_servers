use std::env;

use mongodb::{bson::{doc, Document}, options::ClientOptions, Client, Collection};
use std::convert::Infallible;
use warp::{Filter};
// use futures::stream::TryStreamExt;

//use crate::lib::structs::{ user::User,  typed::{ Result } };
use crate::lib::constants;

// const DB_NAME: String = String::from("real_state_game");  // : &str = "real_state_game";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> mongodb::error::Result<Self> {
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
    
        Ok(Self{
            client
        })
    }

    fn get_collection<T>(&self, col: &str) -> Collection<T> {
        self.client.database(&constants::DB_NAME).collection(col)
    }   
}

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}


// #[derive(Clone, Debug)]
// pub struct UserModel {
//     pub db: DB,
//     pub table_name: String
// }

pub mod user_model {
    // use mongodb::{bson::{Document}};
    use futures::stream::TryStreamExt;

    use crate::lib::structs::{ user::User,  typed::Result };
    use super::DB;

    pub async fn fetch_all(db: DB) -> Result<Vec<User>>{
        let mut cursor = db.get_collection::<User>("user")
            .find(None, None)
            .await.unwrap();

        let mut users: Vec<User> = Vec::new();

        loop {
            match cursor.try_next().await {
                Ok(x) => {
                    match x {
                        Some(d) => {
                            // let user = doc_factory(d);
                            users.push(d);
                        },
                        None => { break }
                    }
                },
                _ => { break }
            }
        }
        Ok(users)
    }

    pub async fn create_one(db:DB, user: User) -> Result<bool> {
        let inserted = db.get_collection("user").insert_one(user, None).await;

        match inserted {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)         
        }
    }

    // pub fn doc_factory(doc: &Document) -> User {
    //     let name = doc.get_str("name").unwrap();

    //     let user = User {
    //         name: name.to_owned()
    //     };

    //     user
    // }
}
