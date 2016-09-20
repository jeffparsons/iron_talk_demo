#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, diesel_codegen)]
#![allow(unused_imports)]

extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate serde_json;
#[macro_use]
extern crate uuid;
extern crate bodyparser;
#[macro_use]
extern crate diesel;

use iron::*;
use staticfile::*;
use mount::*;
use router::*;
use uuid::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

infer_schema!("/tmp/iron_talk_demo.sqlite");

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[insertable_into(submission)]
struct Submission {
    id: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[insertable_into(comment)]
struct Comment {
    id: String,
    // Can be either a submission or another comment
    parent_id: String,
    comment_plaintext: String,
}

#[derive(Clone, Deserialize, Debug)]
struct CreateCommentRequest {
    // Can be either a submission or another comment
    parent_id: String,
    comment_plaintext: String,
}

fn connect() -> SqliteConnection {
    SqliteConnection::establish("/tmp/iron_talk_demo.sqlite").unwrap()
}

fn main() {
    // Sample content
    insert_submission(&Submission {
        id: Uuid::new_v4().to_string(),
        url: "https://www.rust-lang.org".to_owned(),
    });
    insert_submission(&Submission {
        id: Uuid::new_v4().to_string(),
        url: "https://reddit.com".to_owned(),
    });
    insert_submission(&Submission {
        id: Uuid::new_v4().to_string(),
        url: "https://news.ycombinator.com".to_owned(),
    });

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
    let conn = connect();
    let submissions = submission::table.load::<Submission>(&conn).unwrap();
    let res_body = serde_json::to_string(&submissions).unwrap();
    Ok(Response::with((iron::status::Ok, res_body)))
}

fn index_comments(_: &mut Request) -> IronResult<Response> {
    let conn = connect();
    let comments = comment::table.load::<Comment>(&conn).unwrap();
    let res_body = serde_json::to_string(&comments).unwrap();
    Ok(Response::with((iron::status::Ok, res_body)))
}

fn create_comment(req: &mut Request) -> IronResult<Response> {
    let create_request = req.get::<bodyparser::Struct<CreateCommentRequest>>().unwrap().unwrap();
    let new_comment = Comment {
        id: Uuid::new_v4().to_string(),
        parent_id: create_request.parent_id,
        comment_plaintext: create_request.comment_plaintext,
    };
    insert_comment(&new_comment);
    Ok(Response::with((iron::status::Created)))
}

fn insert_submission(new_submission: &Submission) {
    let conn = connect();
    diesel::insert(new_submission)
        .into(submission::table)
        .execute(&conn)
        .unwrap();
}

fn insert_comment(new_comment: &Comment) {
    let conn = connect();
    diesel::insert(new_comment)
        .into(comment::table)
        .execute(&conn)
        .unwrap();
}
