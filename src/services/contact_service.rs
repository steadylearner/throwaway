use mongodb::bson::{
  doc,
  document::Document,
};

use mongodb::{
  error::Error, 
  results::{
    InsertOneResult,
    // InsertManyResult,
  },
  Collection
};

#[derive(Clone)]
pub struct ContactService {
  collection: Collection,
}

impl ContactService  {
  pub fn new(collection: Collection) -> ContactService  {
    ContactService  { collection }
  }

  pub async fn create(&self, email: &str, name: &str) -> Result<InsertOneResult, Error> {
    Ok(self.collection.insert_one(doc! { "email": email, "name": name }, None).await?)
  }

  pub async fn find_by_email(&self, email: &str) -> Result<Option<Document>, Error> {
    // https://github.com/mongodb/mongo-rust-driver#finding-documents-in-a-collection
    let filter = doc! { "email": email };
    Ok(self.collection.find_one(filter, None).await?)
  }
}