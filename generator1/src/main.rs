use crate::expression::models::{AExpr, BinaryOperation};
use crate::expression::{evaluation, models};
use crate::sequence::arithmetic::Arithmetic;
use crate::sequence::models::Sequence;
use crate::sequence::constant::Constant;
use crate::sequence::geometric::Geometric;
use crate::sequence::prod::Produkt;
use crate::sequence::drop::Drop;
use crate::sequence::linear::LinearCombination;
use crate::sequence::potenca::PowerSequence;
use crate::sequence::log::LogSequence;
use crate::sequence::rand::ProbabilisticSequence;
use crate::sequence::operacije::OperationSequence;


use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::convert::Infallible;

pub mod expression;
pub mod sequence;

use std::collections::HashMap;



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
use serde_json::json;



const DEFAULT_PORT: u16 = 9000;
const DEFAULT_IP: &str = "127.0.0.1";



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
    pub name:  String,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceRequest {
    pub range: Range,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
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
        description: "Aritmetično zaporedje. 1. parameter: začetni člen. 2. parameter: diferenca.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Lin_Comb".to_string(),
        description: "Linearna kombinacija dveh zaporedij. Vzame dva parametra a,b ter dve zaporedji f in g. Iz njiju tvori linearno kombinacijo zaporedij a*f + b*g.
        1. parameter: a, 2. parameter: b, 1. zaporedje: f. 2. zaporedje: g".to_string(),
        parameters: 2,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Constant".to_string(),
        description: "Konstantno zaporedje. 1. parameter: vrednost".to_string(),
        parameters: 1,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Product".to_string(),
        description: "Produkt dveh zaporedij. Sprejme prvo in drugo zaporedje in na k-tem mestu vrne zmnožek njunih k-tih členov.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Geometric".to_string(),
        description: "Geometrijsko zaporedje. 1. paramter: začetni člen. 2. parameter: količnik.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "logaritemski".to_string(),
        description: "Logaritemsko zaporedje. Sprejme dve zaporedji. Prvo zaporedje vzame za logaritmant,
        drugega pa za bazo logaritma. Če je logaritem nedefiniran (npr. baza enaka 1, ali negativen logaritmant), vrne tip f64::NAN ".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "potenca".to_string(),
        description: "Eksponentno zaporedje. Prvo zaporedje vzame za osnovo, drugega pa za potenco.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Operacije".to_string(),
        description: "Operacijsko zaporedje. Sprejme 3 zaporedja in 2 parametra. 
        Najprej izračunamo vrednosti prvega in drugega zaporedja ter jih uporabimo v operaciji, ki vrne rezultat a. 
        Nato izračunamo vrednosti prvega in tretjega zaporedja, ki vrne rezultat b.
        Nato primerjamo absolutne razlike med a in parametrom c ter med b in parametrom c. Tisti rezultat, ki je bližje parametru c, se izbere kot vrednost trenutnega člena zaporedja.
        1. zaporedje, 2. zaporedje, 3. zaporedje. 1.parameter: operacija (1 - seštevanje, 2 - odštevanje, 3 - množenje, 4 - deljenje), 2. parameter - c, to je vrednost, s katero bomo primerjali nastale vrednosti in iskali najmanjšo razliko. ".to_string(),
        parameters: 2,
        sequences: 3,
    });
    sequences.push(SequenceInfo {
        name: "Random".to_string(),
        description: "Naključno zaporedje. Sprejme dve zaporedje in en parameter možnosti. Ta parameter naj bo število med 0 in 1 (oboje vključno). Na podlagi izbire parametra
        bo zaporedje izbiralo med členi prvega in drugega zaporeje. Višja kot je številka, bolj verjetno bo izbiralo člene iz prvega zaporedja. (Primer, če za parameter možnosti izberemo število 1, bo vedno podal samo člene prvega zaporedja,
        če za parameter izberemo 0, bo izbiral vedno samo člene drugega zaporedja, če pa izberemo število 0.50, bo podajal obe zaporedji približno enako). ".to_string(),
        parameters: 1,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Drop".to_string(),
        description: "Drop sequence. Sprejme zaporedje in parameter. Glede na parameter izpusti prvih toliko členov, kolikšen je parameter.".to_string(),
        parameters: 1,
        sequences: 1,
    });
    sequences
}
fn get_project() -> Project {
    return Project {
        name: "Anže & Enej".to_string(),
        ip: "127.0.0.1".to_string(),
        port: DEFAULT_PORT,
    };
}

fn get_sequence(sequence_name: &str) -> Option<SequenceInfo> {
    sequences().iter().find(|&seq| seq.name == sequence_name).cloned()
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
    println!("Received body: {}", whole_body); // Debugging line
    Ok(whole_body)
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

async fn register_with_central_register(register_ip: &str, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/generator", register_ip);
    let response = send_post(url, serde_json::to_string(project)?).await?;
    println!("Registration response: {}", response);
    Ok(())
}

pub fn build_sequence_from_syntax(syntax: &SequenceSyntax) -> Box<dyn Sequence<f64>> {
    match syntax.name.as_str() {
        "Constant" => Box::new(Constant::new("Constant".to_string(), syntax.parameters[0])),
        "Arithmetic" => Box::new(Arithmetic::new("Arithmetic".to_string(),syntax.parameters[0],syntax.parameters[1])),
        "Geometric" => Box::new(Geometric::new("Geometric".to_string(),syntax.parameters[0], syntax.parameters[1])),
        "Lin_Comb" => { 
            Box::new(LinearCombination::new(
                syntax.name.clone(),
                build_sequence_from_syntax(&*syntax.sequences[0]),
                build_sequence_from_syntax(&*syntax.sequences[1]),
                syntax.parameters[0],
                syntax.parameters[1],
            )) 
        }
        "Drop" => { 
            Box::new(Drop::new(
                syntax.name.clone(),
                build_sequence_from_syntax(&*syntax.sequences[0]),
                syntax.parameters[0] as usize,
            )) 
        },
        "logaritemski" => { 
            Box::new(LogSequence::new(
                syntax.name.clone(),
                build_sequence_from_syntax(&*syntax.sequences[0]),
                build_sequence_from_syntax(&*syntax.sequences[1]),
            )) 
        }
        "Operacije" => { 
            Box::new(OperationSequence::new(
                syntax.name.clone(),
                build_sequence_from_syntax(&*syntax.sequences[0]),
                build_sequence_from_syntax(&*syntax.sequences[1]),
                build_sequence_from_syntax(&*syntax.sequences[2]),
                syntax.parameters[0],
                syntax.parameters[1],
            )) 
        }
        "potenca" => { 
            Box::new(PowerSequence::new(
                syntax.name.clone(),
                build_sequence_from_syntax(&*syntax.sequences[0]),
                build_sequence_from_syntax(&*syntax.sequences[1]),
            )) 
        }
        "Product" => { 
            Box::new(Produkt::new(
                syntax.name.clone(),
                build_sequence_from_syntax(&*syntax.sequences[0]),
                build_sequence_from_syntax(&*syntax.sequences[1]),
            )) 
        }
        "Random" => { 
            Box::new(ProbabilisticSequence::new(
                syntax.name.clone(),
                build_sequence_from_syntax(&*syntax.sequences[0]),
                build_sequence_from_syntax(&*syntax.sequences[1]),
                syntax.parameters[0]
            )) 
        }
        _ => panic!("Unknown sequence type: {}", syntax.name),
    }
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 9000).into();

    let b = send_get("http://127.0.0.1:7878/project".to_string()).await?;
    println!("HERE {}", b);

    let b = send_post(
        "http://127.0.0.1:7878/project".to_string(),
        serde_json::to_string(&get_project()).unwrap(),
    )
    .await?;
    println!("HERE {}", b);

    let b = send_get("http://127.0.0.1:7878".to_string()).await?;
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
                            println!("Received POST request to path: {}", r);
                            let seqs = sequences();
                            let sequences = seqs
                                .iter()
                                .find(|&x| ("/sequence/".to_string() + &x.name) == r);
                            match sequences {
                                None => {println!("path not found: {}", r); create_404()},
                                Some(s) if *s.name == "Constant".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest =
                                        serde_json::from_str(&body).unwrap();
                                    let range = request.range;
                                    let seq = Constant::new(
                                        "Constant".to_string(),
                                        request.parameters[0],
                                    );
                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                },
                                Some(s) if *s.name == "Arithmetic".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest =
                                        serde_json::from_str(&body).unwrap();
                                    let range = request.range;
                                    let seq = Arithmetic::new(
                                        "Arithmetic".to_string(),
                                        request.parameters[0],
                                        request.parameters[1],
                                    );
                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }, 
                                Some(s) if *s.name == "Geometric".to_string() =>{
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest =
                                        serde_json::from_str(&body).unwrap();s;
                                    let range = request.range;
                                    let seq = Geometric::new(
                                        "Geometric".to_string(),
                                        request.parameters[0],
                                        request.parameters[1],
                                    );
                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                },
                                Some(s) if *s.name == "Lin_Comb".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                    let range = request.range;

                                    let seq1 = build_sequence_from_syntax(&request.sequences[0]);
                                    let seq2 = build_sequence_from_syntax(&request.sequences[1]);
                                
                                    let seq = LinearCombination::new(
                                        "Lin_Comb".to_string(),
                                        seq1, 
                                        seq2,
                                        request.parameters[0], 
                                        request.parameters[1],  
                                    );

                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                Some(s) if *s.name == "Drop".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                    let range = request.range;

                                    let seq1 = build_sequence_from_syntax(&request.sequences[0]);;
                                
                                    let seq = Drop::new(
                                        "Drop".to_string(),
                                        seq1,
                                        request.parameters[0] as usize,  
                                    );

                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                Some(s) if *s.name == "logaritemski".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                    let range = request.range;

                                    let seq1 = build_sequence_from_syntax(&request.sequences[0]);
                                    let seq2 = build_sequence_from_syntax(&request.sequences[1]);
                                
                                    let seq = LogSequence::new(
                                        "logaritemski".to_string(),
                                        seq1, 
                                        seq2,  
                                    );

                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                Some(s) if *s.name == "Operacije".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                    let range = request.range;

                                    let seq1 = build_sequence_from_syntax(&request.sequences[0]);
                                    let seq2 = build_sequence_from_syntax(&request.sequences[1]);
                                    let seq3 = build_sequence_from_syntax(&request.sequences[2]);
                                
                                    let seq = OperationSequence::new(
                                        "Operacije".to_string(),
                                        seq1, 
                                        seq2,  
                                        seq3,
                                        request.parameters[0],
                                        request.parameters[1]

                                    );

                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                Some(s) if *s.name == "potenca".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                    let range = request.range;

                                    let seq1 = build_sequence_from_syntax(&request.sequences[0]);
                                    let seq2 = build_sequence_from_syntax(&request.sequences[1]);
                                                                   
                                    let seq = PowerSequence::new(
                                        "potenca".to_string(),
                                        seq1, 
                                        seq2,  
                                        );

                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                Some(s) if *s.name == "Product".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                    let range = request.range;

                                    let seq1 = build_sequence_from_syntax(&request.sequences[0]);
                                    let seq2 = build_sequence_from_syntax(&request.sequences[1]);
                                                                   
                                    let seq = Produkt::new(
                                        "Product".to_string(),
                                        seq1, 
                                        seq2,  
                                        );

                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                Some(s) if *s.name == "Random".to_string() => {
                                    let body = collect_body(req).await?;
                                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                    let range = request.range;

                                    let seq1 = build_sequence_from_syntax(&request.sequences[0]);
                                    let seq2 = build_sequence_from_syntax(&request.sequences[1]);
                                
                                    let seq = ProbabilisticSequence::new(
                                        "Operacije".to_string(),
                                        seq1, 
                                        seq2,  
                                        request.parameters[0],

                                    );

                                    Ok(Response::new(full(
                                        serde_json::to_string(&seq.range(range)).unwrap(),
                                    )))
                                }
                                _ => {println!("Not implemented for path: {}", r);panic!("Not implemented")},
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
}