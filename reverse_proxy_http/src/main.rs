// use std::convert::Infallible;
use hyper::{Client, Server, Body, Response, Request};
// use futures_util::stream::StreamExt;
use anyhow::*;
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use std::sync::{
    Arc, // Atomically Reference Counted -> https://doc.rust-lang.org/std/sync/struct.Arc.html
    RwLock, // Read-write lock
};  

/*
 From video: 
   Live coding an HTTP reverse proxy in Rust
   https://www.youtube.com/watch?v=FcHYQMRfGWw
 And repo: 
   https://gist.github.com/snoyberg/35a661fff527692d09675ef540c7c1eb
*/

fn mutate_request(req: &mut Request<Body>) -> Result<()>{
    // transfer layer headers
    // We want to change it for other.
    for key in &["content-length", "transfer-encoding", "accept-encoding", "content-encoding"] {
        req.headers_mut().remove(*key);
    }

    let uri = req.uri();
    let uri_string = match uri.query() {
        None => format!("https://www.jaconsta.com{}", uri.path()),
        Some(query) => format!("https://www.jaconsta.com{}?{}", uri.path(), query),
    };
    *req.uri_mut() = uri_string.parse().context("Parsing the URI in mutate_request")?;
    Ok(())
}


#[derive(Debug)]
struct NonProxiedStats {
    proxied: usize,
}

#[tokio::main]
async fn main() -> Result<()>{
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let client: Arc<Client<_, hyper::Body>> = Arc::new(client);
    
    let count_stats: Arc<RwLock<NonProxiedStats>> = Arc::new(RwLock::new(NonProxiedStats{
        proxied: 0,
    }));

    let addr = SocketAddr::from(([0,0,0,0], 3000));

    // It'll run in a single core, single task.
    // MakeService handles each connection
    let make_service = make_service_fn(move |_conn| {  // Move -> generate a "new" service
        let client = Arc::clone(&client);
        let count_stats = Arc::clone(&count_stats);
        async move {

            Ok(service_fn(move |mut req| {
                let client = Arc::clone(&client);
                let count_stats = Arc::clone(&count_stats);

                async move {
                    if req.uri().path() == "/status" {
                        let count_stats = count_stats.read().unwrap();
                        let body: Body = format!("{:?}", &count_stats).into();
                        
                        Ok(Response::new(body))
                    } else {
                        count_stats.write().unwrap().proxied += 1;
                        let client = Arc::clone(&client);

                        mutate_request(&mut req)?;
                        let res = client.request(req).await.context("Making request to backed server")?;
                        // Ok(Response::new(Body::from("Hello World")))
                        Ok(res)
                    }
                }
            }))
        }
    });

    // let url = "htps://httpbin.org/json".parse().context("Parse Url")?;
    // let res = client.get(url).await.context("Perform Http request")?;
    // println!("{:?}", res);
    // println!("---");
    // let body = res.body();
    // println!("{:?}", body);

    Server::bind(&addr).serve(make_service).await.context("Running server")?;
    Ok(())
}
