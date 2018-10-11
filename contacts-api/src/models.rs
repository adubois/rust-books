use diesel;
use diesel::prelude::*;
use diesel::pg:PgConnection;

pub #[derive(Queryable, Debug)]
struct Book {
    id: i32,
    title: String;
    author: String;
    published: bool;
}