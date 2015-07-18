extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;

use iron::prelude::{Request, Response, IronResult, Iron, Chain};
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use staticfile::Static;
use mount::Mount;
use router::Router;
use router::Params;

use std::path::Path;
use time::precise_time_ns;

pub struct RequestTime;
impl typemap::Key for RequestTime {
    type Value = u64;
}

pub struct RequestLogger {
    tag: String
}

impl BeforeMiddleware for RequestLogger {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let start_time = precise_time_ns();

        println!("{} - Started request at: {}, to: {}, from: {}", self.tag, start_time, req.url, req.remote_addr);
        req.extensions.insert::<RequestTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for RequestLogger {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let total_time_nanos = req.extensions.get::<RequestTime>().unwrap();
        println!("{} - Request took: {} ns", self.tag, total_time_nanos);
        Ok(res)
    }
}

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

    let mut chain = Chain::new(mount);
    chain.link_before(RequestLogger{ tag: "before".to_string() });
    chain.link_after(RequestLogger{ tag: "after".to_string() });

    Iron::new(chain)
        .http("localhost:3000")
        .unwrap();
}
