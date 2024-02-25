use juniper::{graphql_object, GraphQLInputObject, GraphQLObject, ID};

#[derive(GraphQLObject)]
struct Book {
    id: ID,
    title: String,
}

#[derive(GraphQLInputObject)]
struct BookInput {
    title: String,
}

pub struct Query;

#[graphql_object]
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

#[graphql_object]
impl Mutation {
    fn create_book(book_input: BookInput) -> Book {
        Book {
            id: ID::new("book-1"),
            title: book_input.title,
        }
    }
}
