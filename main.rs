use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;

mod api;

#[get("/trie/{key}")]
async fn trie_get(
    state: web::Data<api::API>,
    web::Path(key): web::Path<String>,
) -> impl Responder {
    if let Some(value) = state.trie_get(&hex::decode(key).unwrap()) {
        HttpResponse::Ok().body(hex::encode(value))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/validators")]
