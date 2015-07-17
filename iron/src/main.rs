extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

struct MyHandler;

fn handle_request(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn main() {
    Iron::new(handle_request)
        .http("localhost:3000")
        .unwrap();
}
