# First Rust Graphql Server

source: <https://dev.to/open-graphql/building-powerful-graphql-servers-with-rust-3gla>

## Create Server and test

```bsh
cargo new events-graphql-server
cd events-graphql-server
cargo run
```

## Dummy Test Data

Create a file `graphql_schema.rs` in the `src/` directory.

```rs
use juniper::{EmptyMutation,RootNode};

struct Event {
    id: i32,
    kind: String,
    title: String,
    description: String,
    link: String,
    }

#[juniper::object(description = "A member of a team")]
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

    pub struct QueryRoot;

    #[juniper::object]
    impl QueryRoot {
      fn events() -> Vec<Event> {
        vec![
          Event {
            id: 1,
            kind: "MeetupGraphql".to_owned(),
            title: "MeetupGraphql".to_owned(),
            description: "MeetupGraphql".to_owned(),
            link: "MeetupGraphql".to_owned(),

          },
          Event {
            id: 2,
            kind: "React".to_owned(),
            title: "React".to_owned(),
            description: "React".to_owned(),
            link: "React".to_owned(),

          }
        ]
      }
    }
// expose the schema by defining the RootNode type that we expose on our schema.
  pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

    pub fn create_schema() -> Schema {
      Schema::new(QueryRoot {}, EmptyMutation::new())
    }
```

## Main.rs

In `Main.rs` remove the starter function and add

```rs
#[macro_use]
    extern crate juniper;

    use std::io;
    use std::sync::Arc;

    use actix_web::{web, App, Error, HttpResponse, HttpServer};
    use futures::future::Future;
    use juniper::http::graphiql::graphiql_source;
    use juniper::http::GraphQLRequest;

    mod graphql_schema;

    use crate::graphql_schema::{create_schema, Schema};

    fn main() -> io::Result<()> {
        let schema = std::sync::Arc::new(create_schema());
        HttpServer::new(move || {
            App::new()
                .data(schema.clone())
                .service(web::resource("/graphql").route(web::post().to_async(graphql)))
                .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        })
        .bind("localhost:8080")?
        .run()
    }
```

After or before `fn.main` add a handler to `graphql` route.

```rs
   fn graphql(
        st: web::Data<Arc<Schema>>,
        data: web::Json<GraphQLRequest>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        web::block(move || {
            let res = data.execute(&st, &());
            Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
        })
        .map_err(Error::from)
        .and_then(|user| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user))
        })
    }
```

Then a handler for `graphiql`

```rs
 fn graphiql() -> HttpResponse {
        let html = graphiql_source("http://localhost:8080/graphql");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)
    }
```

## Using with Postgres w/ Deisel

Make sure `postgres` is running. Install `deisel` with

```bsh
cargo install diesel_cli --no-default-features --features postgres
```

## add postgres uri to `.env`

Run `echo DATABASE_URL=postgres://localhost/events_graphql_server > .env`

Deisel will pick up the db we want to use from here.

Run `diesel setup` then create `migrations` with `diesel migration generate create_events`

```bsh
➜  events-graphql-server [master*]echo DATABASE_URL=postgres://localhost/events_graphql_server > .env
➜  events-graphql-server [master*]diesel setup
Creating migrations directory at: /Users/prisc_000/working/DEMOS/events-graphql-server/migrations
Creating database: events_graphql_server
➜  events-graphql-server [master*]diesel migration generate create_events
Creating migrations/2019-09-17-203546_create_events/up.sql
Creating migrations/2019-09-17-203546_create_events/down.sql
➜  events-graphql-server [master*]
```

Create `sql` to seed your db:
            kind: "React".to_owned(),
            title: "React".to_owned(),
            description: "React".to_owned(),
            link: "React".to_owned(),
```sql

    CREATE TABLE events (
      id SERIAL PRIMARY KEY,
      title VARCHAR NOT NULL,
      description VARCHAR NOT NULL,
      link VARCHAR NOT NULL,
      kind VARCHAR NOT NULL,
    );

    INSERT INTO events(title, description, link, kind) VALUES ('Link','Description for Link Event', 'www.google.com', 'meetup');
    INSERT INTO events(title, description, link, kind) VALUES ('Mario','Description for Link Event', 'www.google.com', 'meetup');
    INSERT INTO events(title, description, link, kind) VALUES ('Kirby','Description for Link Event', 'www.google.com', 'dinner');
    INSERT INTO events(title, description, link, kind) VALUES ('Ganondorf','Description for Link Event', 'www.google.com', 'planning');
    INSERT INTO events(title, description, link, kind) VALUES ('Bowser','Description for Link Event', 'www.google.com', 'meetup');
    INSERT INTO events(title, description, link, kind) VALUES ('Mewtwo','Description for Link Event', 'www.google.com', 'meetup');

```

In `down.sql` add `DROP TABLE events;` You can destroy the data by running `diesel migration redo`.

Check the generated `schema.rs` file. This is the schema deisel will use.

Run the migrations with `diesel migration run`.

The reason why we named the GraphQL schema file `graphql_schema.rs` instead of `schema.rs`, is because `diesel` overwrites that file in our `src` directory by default.

## Wiring up our Handlers with Diesel

Update your `graphql_schema.rs` file like so:

```rs
use juniper::{EmptyMutation,RootNode};

#[derive(Queryable)]
struct Event {
    pub id: i32,
    pub kind: String,
    pub title: String,
    pub description: String,
    pub link: String,
    }

#[juniper::object(description = "A member of a team")]
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
```

Add a root call for `Events` still in `graphql_schema.rs`

```rs
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
  fn events() -> Vec<Member> {
    use crate::schema::events::dsl::*;
    let connection = establish_connection();
    events
      .limit(100)
      .load::<Member>(&connection)
      .expect("Error loading events")
  }
}
```