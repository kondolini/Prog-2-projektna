use crate::expression::models::{AExpr, BinaryOperation};
use crate::expression::{evaluation, models};
use crate::sequence::arithmetic::Arithmetic;
use crate::sequence::models::Sequence;
use crate::sequence::constant::Constant;

pub mod expression;
pub mod sequence;

use std::collections::HashMap;

use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Error;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use serde::{Deserialize, Serialize};

const PORT: u16 = 12345;



#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub from: u64,
    pub to: u64,
    pub step: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceSyntax {
    pub name: String,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceRequest {
    pub range: Range,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceInfo {
    name: String,
    description: String,
    parameters: u32,
    sequences: u32,
}

fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Lin Comb".to_string(),
        description: "".to_string(),
        parameters: 3,
        sequences: 2,
    });
    sequences
}
fn get_project() -> Project {
    return Project {
        name: "Anže & Enej".to_string(),
        ip: "0.0.0.0".to_string(),
        port: PORT,
    };
}

fn get_sequence(sequence: &dyn Sequence<f64>) -> SequenceInfo {
    return SequenceInfo {
    name: sequence.name().clone(),
    description: format!("To je zaporedje {}", sequence.name()),
    parameters: 2,
    sequences: 1,

    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
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

async fn send_post(url: String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    return Ok(res);
}

async fn send_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.text().await?;
    return Ok(res);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], PORT).into();

    let b = send_get("http://0.0.0.0:7878/project".to_string()).await?;
    println!("HERE {}", b);

    let b = send_post(
        "http://0.0.0.0:7878/project".to_string(),
        serde_json::to_string(&get_project()).unwrap(),
    )
    .await?;
    println!("HERE {}", b);

    let b = send_get("http://0.0.0.0:7878".to_string()).await?;
    println!("HERE {}", b);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let create_404 = || {
        let mut not_found = Response::new(empty());
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/ping") => Ok::<_, Error>(Response::new(full(
                            serde_json::to_string(&get_project()).unwrap(),
                        ))),
                        (&Method::GET, "/sequence") => {
                            //
                            let sequences = sequences();
                            Ok(Response::new(full(
                                serde_json::to_string(&sequences).unwrap(),
                            )))
                        }
                        (&Method::POST, r) => {
                            let seqs = sequences();
                            let sequences = seqs
                                .iter()
                                .find(|&x| ("/sequence/".to_string() + &x.name) == r);
                            match sequences {
                                None => create_404(),
                                Some(s) if *s.name == "Arithmetic".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest =
                                        serde_json::from_str(&body).unwrap();
                                    let range = request.range;
                                    let seq = Arithmetic::new(
                                        request.parameters[0],
                                        request.parameters[1],
                                    );
                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                _ => panic!("Not implemented"),
                            }
                        }

                        _ => create_404(),
                    }
                }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
    let seq = Arithmetic::new(0.0, 1.0);
    let range = Range { from: 0, to: 5, step: 1 };
    let result = seq.range(range);
    println!("{:?},aaaaaaaaaa", result);
}

