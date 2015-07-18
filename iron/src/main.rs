extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

use iron::prelude::{Request, Response, IronResult, Iron};
use iron::status;
use staticfile::Static;
use std::path::Path;
use mount::Mount;
use router::Router;
use router::Params;

fn hello_world(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn handle_get_with_path_variables(req: &mut Request) -> IronResult<Response> {
    let params: &Params = req.extensions.get::<Router>().unwrap();
    params.find("thing")
        .and_then(|thing_param| params.find("adjective").map(|adj| (thing_param, adj)))
        .map(|(thing, adjective)| format!("{} is like so totally {}!", thing, adjective))
        .map(|response_string| Ok(Response::with((status::Ok, response_string))))
        .unwrap()
}

fn main() {
    let static_file_handler = Static::new(Path::new("static"));

    let mut router = Router::new();
    router.get("/:thing/:adjective", handle_get_with_path_variables);

    let mut mount = Mount::new();
    mount.mount("/hello", hello_world)
        .mount("/static", static_file_handler)
        .mount("/routes", router);

    Iron::new(mount)
        .http("localhost:3000")
        .unwrap();
}
