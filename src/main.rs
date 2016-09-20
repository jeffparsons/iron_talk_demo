extern crate iron;
extern crate staticfile;

use iron::*;
use staticfile::*;


fn main() {
    let static_handler = Static::new("web/");

    let addr = "0.0.0.0:8888";
    println!("Listening on {}...", addr);
    println!("Try http://{}!", addr);
    Iron::new(static_handler).http(addr).unwrap();
}
