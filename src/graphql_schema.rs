extern crate dotenv;
use crate::schema::todos;

use diesel::pg::PgConnection;
use diesel::Insertable;

use diesel::prelude::*;
use dotenv::dotenv;
use juniper::RootNode;
use std::env;

#[derive(Queryable)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[juniper::object(description = "A todo")]
impl Todo {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

pub struct QueryRoot;
pub struct MutationRoot;

#[juniper::object]
impl QueryRoot {
    fn todos() -> Vec<Todo> {
        use crate::schema::todos::dsl::*;

        let connection = &mut establish_connection();
        let results = todos.load::<Todo>(connection).expect("Error loading");

        results
    }
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
    pub completed: bool,
}

#[juniper::object]
impl MutationRoot {
    fn create_todo(new_todo: NewTodo) -> Todo {
        use crate::schema::todos::dsl::*;
        let connection = &mut establish_connection();

        let todo = diesel::insert_into(todos)
            .values(&new_todo)
            .get_result(connection)
            .expect("Error saving new todo");

        todo
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
