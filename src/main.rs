use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode, ID};
use rocket::{get, launch, post, response::content::RawHtml, routes, Build, Rocket, State};

#[derive(GraphQLObject)]
struct Book {
    id: ID,
    title: String,
}

struct Query;

#[graphql_object]
impl Query {
    fn books() -> Vec<Book> {
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

type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

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
    rocket::build()
        .manage(Schema::new(
            Query,
            EmptyMutation::new(),
            EmptySubscription::new(),
        ))
        .mount("/", routes![get_graphql, post_graphql, playground])
}
