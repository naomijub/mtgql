#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate juniper;
extern crate serde;
extern crate hyper;
extern crate juniper_hyper;
extern crate futures;
extern crate futures_cpupool;
extern crate pretty_env_logger;

use std::fs::File;
use std::io::prelude::*;
use futures::future;
use futures_cpupool::CpuPool;
use hyper::rt::{Future};
use hyper::{Body, Method, Response, Server, StatusCode};
use hyper::service::{service_fn};
use juniper::{FieldResult, EmptyMutation};
use juniper::RootNode;
use std::sync::Arc;

fn main() {
    pretty_env_logger::init();
    let pool = CpuPool::new(4);
    let addr = ([127, 0, 0, 1], 3000).into();

    let root_node = Arc::new(RootNode::new(Arc::new(Query), EmptyMutation::<Card>::new()));

    let service = move || {
        let root_node = root_node.clone();
        let pool = pool.clone();
        let ctx = Arc::new(fake_ctx());
        service_fn(move |req| -> Box<Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            let pool = pool.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
                (&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(pool, root_node, ctx, req)),
                (&Method::POST, "/graphql") => {
                    Box::new(juniper_hyper::graphql(pool, root_node, ctx, req))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

fn read_fake() -> std::io::Result<String> {
    let mut file = File::open("./body.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn fake_ctx() -> Card {
    let v: Card = serde_json::from_str(read_fake().unwrap().as_str()).unwrap();

    v
}

struct Query;

graphql_object!(Query: Card |&self| {
    field allCards(&executor) -> FieldResult<Vec<CardBody>> {
        Ok(executor.context().cards.to_owned())
    }
});


#[derive(Serialize, Deserialize, Debug, Clone, GraphQLObject)]
#[graphql(description="Cards Vector")]
struct Card {
    cards: Vec<CardBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone, GraphQLObject)]
#[graphql(description="Card Fields")]
#[allow(non_snake_case)]
struct CardBody {
    name: String,
    manaCost: Option<String>,
    cmc: i32,
    colors: Vec<String>,
    colorIdentity: Option<Vec<String>>,
    types: Vec<String>,
    subtypes: Option<Vec<String>>,
    rarity: String,
    set: String,
    setName: Option<String>,
    text: String,
    artist: String,
    number: String,
    power: Option<String>,
    toughness: Option<String>,
    layout: String,
    multiverseid: i32,
    imageUrl: Option<String>,
    rulings: Option<Vec<Rulings>>,
    printings: Vec<String>,
    originalText: Option<String>,
    originalType: Option<String>,
    id: String
}

#[derive(Serialize, Deserialize, Debug, Clone, GraphQLObject)]
#[graphql(description="Card Rulings")]
struct Rulings {
    date: String,
    text: String,
}
