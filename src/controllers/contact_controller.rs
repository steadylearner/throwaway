use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use born::{
    nested_macro,
    public_struct,
};

public_struct!(
  pub struct ContactBase {
    email: String,
  }
);

ContactBase!(
  #[derive(Deserialize)]
  pub struct ContactCreateRequest {
    name: String,
  }
);

ContactBase!(
  #[derive(Deserialize)]
  pub struct ContactGetRequest
);

pub async fn create(
  app_data: web::Data<crate::AppState>,
  contact: web::Query<ContactCreateRequest>,
) -> impl Responder {
  let new_contact = app_data.service_container.contact.create(&contact.name, &contact.email).await;

  match new_contact {
    Ok(result) => {
      println!("{:#?}", &result);

      HttpResponse::Ok().json(result.inserted_id) // Return contacts data instead of this.
    },
    Err(e) => {
      println!("Error while creating a contact, {:#?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn find_by_email(
  app_data: web::Data<crate::AppState>,
  contact: web::Query<ContactGetRequest>,
) -> impl Responder {
  let old_contact = app_data.service_container.contact.find_by_email(&contact.email).await;

  match old_contact {
    Ok(result) => { // result is Option here.
      if let Some(payload) = result {
        HttpResponse::Ok().json(payload)
      } else {
        HttpResponse::NotFound().finish()
      }
    }
    Err(e) => {
      println!("Error while finding a contact by email, {:#?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
