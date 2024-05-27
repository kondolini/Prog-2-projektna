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


fn main() {}