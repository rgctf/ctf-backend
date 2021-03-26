use butane::{ForeignKey, ObjectState, model};
use chrono::NaiveDateTime;

#[model]
#[derive(Debug, Default)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub start_timestamp: Option<NaiveDateTime>,
    pub end_timestamp: Option<NaiveDateTime>,
}

impl From<backend_models::Event> for Event {
    fn from(event: backend_models::Event) -> Self {
        Self {
            id: event.id,
            name: event.name,
            start_timestamp: event.start_timestamp,
            end_timestamp: event.end_timestamp,
            ..Default::default()
        }
    }
}

impl Into<backend_models::Event> for Event {
    fn into(self) -> backend_models::Event {
        backend_models::Event {
            id: self.id,
            name: self.name,
            start_timestamp: self.start_timestamp,
            end_timestamp: self.end_timestamp,
        }
    }
}

#[model]
#[derive(Debug)]
pub struct EventChallenge {
    pub id: String,
    pub event: ForeignKey<Event>,
    state: ObjectState,
}

impl EventChallenge {
    pub fn new(event: &Event, id: String) -> Self {
        Self {
            id,
            event: event.into(),
            state: Default::default(),
        }
    }
}
