extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;
extern crate bodyparser;
extern crate rustc_serialize;
extern crate persistent;

use iron::prelude::*;
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
        let start_time = get_current_time_micros();

        println!("{} - Started request at: {}, to: {}, from: {}", self.tag, start_time, req.url, req.remote_addr);
        req.extensions.insert::<RequestTime>(start_time);
        Ok(())
    }
}

impl AfterMiddleware for RequestLogger {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let start_time: u64 = *req.extensions.get::<RequestTime>().unwrap();
        println!("{} - Request took: {} microseconds", self.tag, get_current_time_micros() - start_time);
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        let start_time: u64 = *req.extensions.get::<RequestTime>().unwrap();
        println!("{} - Error Request took: {} microseconds", self.tag, get_current_time_micros() - start_time);
        Err(err)
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

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Beer {
    // id: String,
    name: String,
    abv: f32,
    brewery: Option<String>
}

fn handle_beer_post(req: &mut Request) -> IronResult<Response> {
    let deserialize_result: Result<Option<Beer>, bodyparser::BodyError> = req.get::<bodyparser::Struct<Beer>>();

    match deserialize_result {
        Ok(Some(beer)) => Ok(Response::with((status::Ok, format!("Got beer {:?}", beer)))),
        _ => Ok(Response::with((status::BadRequest, "invalid json")))
    }
}

fn main() {
    let static_file_handler = Static::new(Path::new("static"));

    let mut router = Router::new();
    router.get("/:thing/:adjective", handle_get_with_path_variables);
    router.post("/beer", handle_beer_post);

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

fn get_current_time_micros() -> u64 {
    precise_time_ns() / 1_000u64
}
