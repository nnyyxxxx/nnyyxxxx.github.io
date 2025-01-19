use crate::handlers;
use actix_web::web;

pub fn config() -> actix_web::Scope {
    web::scope("").service(handlers::index)
}
