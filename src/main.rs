use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode, Variables,
};
use rocket::{get, launch, routes, Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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

#[get("/execute")]
async fn execute() {
    let (result, _error) = juniper::execute(
        "
        query Person {
            person {
                name
                age
            }
        }
        ",
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &(),
    )
    .await
    .unwrap();

    println!("{:?}", result);
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, execute])
}
