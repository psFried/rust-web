extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

use iron::prelude::{Request, Response, IronResult, Iron};
use iron::status;
use staticfile::Static;
use std::path::Path;
use mount::Mount;

fn handle_request(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn main() {
    let mut static_file_handler = Static::new(Path::new("static"));

    let mut mount = Mount::new();
    mount.mount("/", handle_request)
        .mount("/static", static_file_handler);

    Iron::new(mount)
        .http("localhost:3000")
        .unwrap();
}
