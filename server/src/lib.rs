use actix_web::{web, App, HttpServer, middleware};
use sea_orm::{Database, DbConn};
use std::env;

pub mod handlers;
pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	
}