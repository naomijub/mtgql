#[macro_use] extern crate serde_derive;
extern crate hyper;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate futures;

use std::fs::File;
use std::io::prelude::*;

use hyper::rt::Future;
use hyper::{Request, Body, Response, Server};
use hyper::service::{service_fn_ok};


fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let ping_svc = move || {
        service_fn_ok(fake_json)
    };



    let server = Server::bind(&addr)
        .serve(ping_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

fn read_fake() -> std::io::Result<String> {
    let mut file = File::open("./body.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn fake_json(_req: Request<Body>) -> Response<Body> {
    let v: Card = serde_json::from_str(read_fake().unwrap().as_str()).unwrap();

    Response::new(Body::from(json!(v.clone()).to_string()))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
    cards: Vec<CardBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CardBody {
    name: String,
    mana_cost: Option<String>,
    cmc: i32,
    colors: Vec<String>,
    color_identity: Option<Vec<String>>,
    types: Vec<String>,
    subtypes: Option<Vec<String>>,
    rarity: String,
    set: String,
    set_name: Option<String>,
    text: String,
    artist: String,
    number: String,
    power: Option<String>,
    toughness: Option<String>,
    layout: String,
    multiverseid: i32,
    image_url: Option<String>,
    rulings: Option<Vec<Rulings>>,
    printings: Vec<String>,
    original_text: Option<String>,
    original_type: Option<String>,
    id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Rulings {
    date: String,
    text: String,
}
