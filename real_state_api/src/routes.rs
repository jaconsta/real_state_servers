use mongodb::Client;
use serde_json::{json, Value};
use warp::{Filter, reply::{ Json, json, with_status, WithStatus }, Rejection, http::StatusCode};

use crate::lib::db::{DB, with_db, user_model};
use crate::lib::structs::user::User;

// FIXME https://github.com/zupzup/rust-web-mongodb-example/tree/main/src
// https://www.mongodb.com/community/forums/t/rust-data-structure-example/6021/3
// https://docs.rs/mongodb/latest/mongodb/

/*
  Rust Linz, October 2021 - Tokio, Warp, and Websockets by Stefan Baumgartner
  https://www.youtube.com/watch?v=fuiFycJpCBw
  https://github.com/ddprrt/warp-websockets-example/blob/main/src/main.rs

  How to Build an API with Rust using Warp and Tokio
  https://www.youtube.com/watch?v=R8i6XKmR2aE

  Rust Web Development - Warp Introduction (by example)
  https://www.youtube.com/watch?v=HNnbIW2Kzbc
  https://github.com/jeremychone-channel/rust-warp-intro/blob/main/src/main.rs

 */ 
fn users_api(db_client: Client) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let users_base = warp::path("api").and(warp::path("v1")).and(warp::path("users"));
    let db: DB = DB{client: db_client };
    
    let list = users_base
		.and(warp::get())
		.and(warp::path::end())
        .and(with_db(db.clone()))
		//.map(|| format!("Hello, users!"));
        .and_then(users_list);

    let create = users_base
		.and(warp::post())
		.and(warp::path::end())
        .and(with_db(db.clone()))
		.and(warp::body::json())
        .and_then(users_create);

    list.or(create)
}

// fn with_db(db_pool: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
//     warp::any().map(move || db_pool.clone())
// }

pub fn routes_api (db: Client) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let hello =  warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    

    // let files = warp::fs::dir("./static");

    let res_404 = warp::any().map(|| {
        let error = json!({ "error": "Not Found" });
        Ok(warp::reply::with_status(
            warp::reply::json(&error),
            warp::http::StatusCode::NOT_FOUND,
        ))
    });

    let routes = hello.or(users_api(db.clone())).or(res_404)
        .with(warp::cors().allow_any_origin());

    routes
}

async fn users_list(db: DB) -> Result<Json, Rejection>{
    let result = user_model::fetch_all(db).await;
    let result = match result{
        Ok(x) => x,
        _ => {
            let users = json!([]);
            return Ok(warp::reply::json(&users))
        }
    };
    
    Ok(json(&result))
}

async fn users_create(_col: DB, data: User)  -> Result<WithStatus<Json>, Rejection>{
    // let users = json!([
	// 	{"id": 1, "name": "Javier"},
	// 	{"id": 2, "name": "Antonio"},
    //     data
	// ]);

    let response = user_model::create_one(_col, data).await;

    if let Err(_) = response {
        let not_created = json!({ "status": "failed to create", "error": "User not created" });
        let not_created = warp::reply::json(&not_created);
        return Ok(with_status(not_created, StatusCode::INTERNAL_SERVER_ERROR))
    }

    let created = json!({ "status": "created" });
	let created = warp::reply::json(&created);

	Ok(with_status(created, StatusCode::CREATED))
}