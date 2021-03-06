#[macro_use]
extern crate dotenv_codegen;
use dotenv::dotenv;
// use std::env;

use actix_web::{
  web::{
    get,
    post,
  },
  App, 
  HttpServer, 
  // Responder
};

use mongodb::{Client, options::ClientOptions };

// use anyhow::{Context, Result};

// Below this line are local folders and files.
mod controllers;
mod services;

use services::{
  contact_service::ContactService,
};

use controllers::{
  contact_controller,
};

#[derive(Clone)]
pub struct ServiceContainer {
  contact: ContactService,
}

impl ServiceContainer {
  pub fn new(contact: ContactService) -> Self {
    ServiceContainer { contact }
  }
}

pub struct AppState {
  service_container: ServiceContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
// async fn main() -> std::io::Result<()> { 
  dotenv().ok();

  let target = "127.0.0.1:8000";
  println!("Actix will be ready at {}", target);

  // It is difficult extract this to function because types used here are private.
  // You can still extract this with a macro.
  let client_options = ClientOptions::parse(dotenv!("MONGODB_LOCAL"))
    .await.expect("There was a problem with mongodb client options."); // ? won't work here

  let client = Client::with_options(client_options)
    .expect("Failed to making a mongodb client."); // ? won't work here.

  let db = client.database("rust");
  
  let contact_collection = db.collection("contact");

  let service_container = ServiceContainer::new(ContactService::new(contact_collection.clone()));
  
  HttpServer::new(move || {
    App::new()
      .data(AppState {
        service_container: service_container.clone(),
      })
      .route("/create", post().to(contact_controller::create)) // Make /api/v1/contatct prefix later.
      .route("/find_by_email", get().to(contact_controller::find_by_email)) // Make /api/v1/contatct prefix later.
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await

}
