extern crate hyper;

use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

static TEXT: &str = "pong";

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let ping_svc = || {
        service_fn_ok(|_req|{
            Response::new(Body::from(TEXT))
        })
    };

    let server = Server::bind(&addr)
        .serve(ping_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
