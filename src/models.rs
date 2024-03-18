use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Book {
    pub _id: ObjectId,
    pub title: String,
}

#[derive(Serialize)]
pub struct NewBook {
    pub title: String,
}

pub struct Link {
    pub _id: ObjectId,
    pub url: String,
}
