use juniper::{graphql_object, GraphQLObject, ID};

#[derive(GraphQLObject)]
struct Book {
    id: ID,
    title: String,
}

pub struct Query;

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
