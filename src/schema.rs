use juniper::{graphql_object, GraphQLInputObject, GraphQLObject, ID};
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use serde::Serialize;
use std::env;

#[derive(GraphQLObject)]
struct Book {
    id: ID,
    title: String,
}

#[derive(GraphQLInputObject, Serialize)]
struct BookInput {
    title: String,
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn get_books() -> Vec<Book> {
        vec![
            Book {
                id: ID::new("book-1"),
                title: String::from("Harry Potter and the Philosopher's Stone"),
            },
            Book {
                id: ID::new("book-2"),
                title: String::from("Harry Potter and the Chamber of Secrets"),
            },
        ]
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_book(book_input: BookInput, context: &Context) -> Book {
        let collection = context.database.collection::<BookInput>("books");
        let result = collection.insert_one(&book_input, None).await.unwrap();
        println!("Inserted a document with _id: {}", result.inserted_id);

        Book {
            id: ID::new("book-1"),
            title: book_input.title,
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
