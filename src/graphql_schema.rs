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

  pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

    pub fn create_schema() -> Schema {
      Schema::new(QueryRoot {}, EmptyMutation::new())
    }