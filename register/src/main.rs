use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Error;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

type Db = Arc<Mutex<HashMap<String, PublicProject>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicProject {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

async fn collect_body(req: Request<Incoming>) -> Result<String, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        panic!("Body too big");
    }

    let whole_body = req.collect().await?.to_bytes();
    let whole_body = std::str::from_utf8(&whole_body).unwrap().to_string();
    return Ok(whole_body);
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 7878).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let db = db.clone();

        let service = service_fn(move |req| {
            let db: Arc<Mutex<HashMap<String, PublicProject>>> = db.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, "/project") => {
                        let db = db.lock().unwrap();
                        let values = db.values().collect::<Vec<_>>();
                        let value = serde_json::to_string(&values).unwrap();
                        println!("Returning: {:?}", value);
                        Ok::<_, Error>(Response::new(full(value)))
                    }
                    (&Method::POST, "/project") => {
                        //
                        let body = collect_body(req).await?;
                        let project: PublicProject = serde_json::from_str(&body).unwrap();
                        let mut db = db.lock().unwrap();
                        db.insert(project.name.clone(), project);
                        println!("Got: {:?}", db);
                        Ok(Response::new(full("")))
                    }
                    _ => {
                        let mut not_found = Response::new(empty());
                        *not_found.status_mut() = StatusCode::NOT_FOUND;
                        Ok(not_found)
                    }
                }
            }
        });

        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}