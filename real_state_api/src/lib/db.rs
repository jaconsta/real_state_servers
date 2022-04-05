use std::env;

use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use std::convert::Infallible;
use warp::{Filter};

use crate::lib::constants;


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


pub mod user_model {
    use futures::stream::TryStreamExt;
    use mongodb::{bson::doc};
    use sha2::{Sha256, Digest};
    use base64ct::{Base64, Encoding};
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    use crate::lib::structs::{ user::User, typed::Result, auth::UserLogin };
    use super::DB;
    use super::constants::HASH_SALT;

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


    pub async fn fetch_by_username(db:DB, username: &str) -> User {
        let mut cursor = db.get_collection::<User>("user")
            .find(doc! {"username": username}, None)
            .await.unwrap();
        let user: User = match cursor.try_next().await {
            Ok(x) => {
                match x {
                    Some(d) => { 
                        return d
                    },
                    None => { User{ id: None, username: String::from(""), first_name: None, last_name: None, email: None, phone: None, password: None ,pss_hash: None} }
                }
            },
            _ => { User{ id: None, username: String::from(""), first_name: None, last_name: None, email: None, phone: None, password: None ,pss_hash: None} }
        };
        user
    }

    pub async fn create_one(db:DB, user: &mut User) -> Result<bool> {
        let custom_hash = super::user_model::hash_generator();
        user.password = match &user.password {
            Some(password) => Some(super::user_model::password_hasher(&password, &custom_hash)),
            _ => None  // Should throw an error
        };
        user.pss_hash = Some(custom_hash);
        
        let inserted = db.get_collection::<User>("user").insert_one(user, None).await;

        match inserted {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)         
        }
    }

    // password_hasher("I am a secure password asdf", "SuperSecureSalt");
    fn password_hasher (password: &str, salt_string: &str) -> String {
        // https://www.cs.rit.edu/~ark/20090927/Round2Candidates/Shabal.pdf
        // Probably check https://github.com/RustCrypto/password-hashes
        let hasher = Sha256::new()
            .chain_update(salt_string.as_bytes())
            .chain_update(HASH_SALT)
            .chain_update(password)
            .finalize();
        let string_hash = Base64::encode_string(&hasher);
        string_hash
    }
    
    fn hash_generator () -> String  {
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
        rand_string
    }

    pub async fn user_validator (db: DB, data: UserLogin) -> Result<User> {
        let mut user = super::user_model::fetch_by_username(db, &data.username).await;

        if user.username == "".to_string() {
            return Ok(User{ id: None, username: String::from(""), first_name: None, last_name: None, email: None, phone: None, password: None ,pss_hash: None});
        }

        let guess_password =  super::user_model::password_hasher(&data.password, &user.pss_hash.clone().unwrap());

        if guess_password.eq(user.password.as_ref().unwrap()) {
            user.password = None;
            return Ok(user.to_owned()) 
        }
        Ok(User{ id: None, username: String::from(""), first_name: None, last_name: None, email: None, phone: None, password: None ,pss_hash: None})
        
    }
}
