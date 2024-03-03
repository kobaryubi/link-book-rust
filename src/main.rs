use juniper::{EmptySubscription, RootNode};
use link_book_rust::schema::{Mutation, Query};
use rocket::{get, launch, post, response::content::RawHtml, routes, Build, Rocket, State};
use std::env;

type Schema = RootNode<'static, Query, Mutation, EmptySubscription>;

#[get("/graphql?<request..>")]
async fn get_graphql(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, &()).await
}

#[post("/graphql", data = "<request>")]
async fn post_graphql(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, &()).await
}

#[get("/playground")]
fn playground() -> RawHtml<String> {
    juniper_rocket::playground_source("/graphql", None)
}

#[launch]
fn rocket() -> Rocket<Build> {
    let link_book_env = env::var("LINK_BOOK_ENV").unwrap_or(String::from("dev"));
    dotenv::from_filename(format!(".env.{}", link_book_env)).expect("Failed to read .env file");

    rocket::build()
        .manage(Schema::new(Query, Mutation, EmptySubscription::new()))
        .mount("/", routes![get_graphql, post_graphql, playground])
}
