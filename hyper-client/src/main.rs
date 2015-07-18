extern crate hyper;

use std::env;
use std::io;

use hyper::Client;
use hyper::header::Connection;
use hyper::Url;
use hyper::mime::*;
use hyper::header::{Accept, qitem};

fn main() {
    env::args().nth(1).map(|url| get_request(url));
}

fn get_request(url: String) {
   match Url::parse(&url) {
       Ok(parsed_url) => perform_request(parsed_url),
       Err(er) => println!("Could not parse url: {}", url)
   }

}

fn perform_request(parsed_url: Url) {
    let client = Client::new();

    let mut res = client.get(parsed_url)
      .header(Connection::close())
      .header(Accept(vec!(qitem(Mime(TopLevel::Star, SubLevel::Star, vec![])))))
      .send().unwrap();

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    io::copy(&mut res, &mut io::stdout()).unwrap();
}
