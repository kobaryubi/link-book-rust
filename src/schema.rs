use crate::models::{Book, Link, NewBook, NewLink};
use futures::stream::TryStreamExt;
use juniper::{graphql_object, FieldError, FieldResult, GraphQLInputObject, GraphQLObject, ID};
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use serde::Serialize;
use std::env;

#[graphql_object]
impl Link {
    fn id(&self) -> ID {
        ID::new(&self._id.to_hex())
    }

    fn url(&self) -> &str {
        &self.url
    }
}

#[graphql_object]
impl Book {
    fn id(&self) -> ID {
        ID::new(&self._id.to_hex())
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn links(&self) -> Vec<Link> {
        vec![Link {
            _id: ObjectId::new(),
            url: String::from("https://www.rust-lang.org/"),
        }]
    }
}

#[derive(GraphQLInputObject, Serialize)]
struct BookInput {
    title: String,
    urls: Vec<String>,
}

impl BookInput {
    fn into_new_book(self, link_ids: Vec<ObjectId>) -> NewBook {
        NewBook {
            title: self.title,
            link_ids,
        }
    }
}

#[derive(GraphQLObject)]
struct BookPayload {
    id: ID,
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn book(id: ID, context: &Context) -> FieldResult<Book> {
        let collection = context.database.collection::<Book>("books");
        let result = collection
            .find_one(doc! {"_id": ObjectId::parse_str(id.to_string())? }, None)
            .await?;

        if let Some(book) = result {
            Ok(book)
        } else {
            Err(FieldError::new("Book not found", "NOT_FOUND".into()))
        }
    }

    async fn books(context: &Context) -> FieldResult<Vec<Book>> {
        let collection = context.database.collection::<Book>("books");
        let cursor = collection.find(None, None).await?;
        let books = cursor.try_collect().await?;

        Ok(books)
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_book(book_input: BookInput, context: &Context) -> FieldResult<BookPayload> {
        let links_collection = context.database.collection::<NewLink>("links");
        let new_links = book_input
            .urls
            .iter()
            .map(|url| NewLink {
                url: url.to_string(),
            })
            .collect::<Vec<_>>();
        let links_insert_many_result = links_collection.insert_many(new_links, None).await?;

        let collection = context.database.collection::<NewBook>("books");
        let result = collection
            .insert_one(
                &book_input.into_new_book(
                    links_insert_many_result
                        .inserted_ids
                        .into_iter()
                        .map(|(_, id)| id.as_object_id().unwrap())
                        .collect(),
                ),
                None,
            )
            .await?;

        if let Bson::ObjectId(oid) = result.inserted_id {
            Ok(BookPayload {
                id: ID::new(oid.to_hex()),
            })
        } else {
            Err(FieldError::new(
                "Failed to insert book",
                "INTERNAL_SERVER_ERROR".into(),
            ))
        }
    }
}

pub struct Context {
    #[allow(dead_code)]
    database: mongodb::Database,
}
impl juniper::Context for Context {}
impl Context {
    pub async fn build() -> mongodb::error::Result<Self> {
        let mut client_options =
            ClientOptions::parse(env::var("MONGODB_CONNECTION_STRING").unwrap()).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        Ok(Context {
            database: client.database("link_book"),
        })
    }
}
