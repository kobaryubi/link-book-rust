use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
use rocket::{get, launch, post, response::content::RawHtml, routes, Build, Rocket, State};
use std::sync::atomic::{AtomicUsize, Ordering};

struct HitCount {
    count: AtomicUsize,
}

#[get("/count")]
fn count(hit_count: &State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    format!("Number of visits: {}", current_count)
}

#[derive(GraphQLObject)]
struct Person {
    name: String,
    age: i32,
}

struct Query;

#[graphql_object]
impl Query {
    fn person() -> Person {
        Person {
            name: "John".to_string(),
            age: 30,
        }
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
        .manage(HitCount {
            count: AtomicUsize::new(0),
        })
        .manage(Schema::new(
            Query,
            EmptyMutation::new(),
            EmptySubscription::new(),
        ))
        .mount("/", routes![count, get_graphql, playground, post_graphql])
}
