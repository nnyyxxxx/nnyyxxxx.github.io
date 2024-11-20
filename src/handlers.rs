use crate::constants::*;
use actix_web::{get, HttpResponse, Responder};
use std::fs;

#[get("/")]
pub async fn index() -> impl Responder {
    let content = fs::read_to_string(INDEX_HTML).unwrap();
    HttpResponse::Ok().body(content)
}
