extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;

use iron::*;
use staticfile::*;
use mount::*;
use router::*;

fn main() {
    let static_handler = Static::new("web/");

    let mut api_handler = Router::new();
    api_handler.get("/submissions", index_submissions, "is");
    api_handler.get("/comments", index_comments, "ic");
    api_handler.post("/comments", create_comment, "cc");

    let mut mount = Mount::new();
    mount.mount("/api", api_handler);
    mount.mount("/", static_handler);

    let addr = "0.0.0.0:8888";
    println!("Listening on {}...", addr);
    println!("Try http://{}!", addr);
    Iron::new(mount).http(addr).unwrap();
}

fn index_submissions(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "[]")))
}

fn index_comments(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "[]")))
}

fn create_comment(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Created)))
}
