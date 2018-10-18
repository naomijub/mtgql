#[macro_use] extern crate serde_derive;
#[macro_use] extern crate juniper;
extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate juniper_hyper;
extern crate futures;
extern crate futures_cpupool;
extern crate pretty_env_logger;
extern crate rayon;

mod client;
mod schema;
mod gqlerror;

use futures::future;
use futures_cpupool::CpuPool;
use hyper::rt::{Future};
use hyper::{Body, Method, Response, Server, StatusCode};
use hyper::service::{service_fn};
use juniper::{EmptyMutation};
use juniper::RootNode;
use std::sync::Arc;
use client::fake_ctx;
use schema::{Card, Query};

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
