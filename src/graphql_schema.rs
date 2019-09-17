extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::{EmptyMutation,RootNode};

use crate::schema::events;

pub struct QueryRoot;

fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[juniper::object]
impl QueryRoot {
  fn events() -> Vec<Event> {
    use crate::schema::events::dsl::*;
    let connection = establish_connection();
    events
      .limit(100)
      .load::<Event>(&connection)
      .expect("Error loading events")
  }
}

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub link: String,
    pub kind: String,
  }

#[juniper::object(description = "A Event of a team")]
    impl Event {
      pub fn id(&self) -> i32 {
        self.id
      }
      pub fn kind(&self) -> &str {
        self.kind.as_str()
      }
      pub fn title(&self) -> &str {
        self.title.as_str()
      }
      pub fn description(&self) -> &str {
        self.description.as_str()
      }
      pub fn link(&self) -> &str {
        self.link.as_str()
      }
    }

  pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

    pub fn create_schema() -> Schema {
      Schema::new(QueryRoot {}, EmptyMutation::new())
    }