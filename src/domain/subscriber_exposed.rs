use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct SubscriberExposed {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Serialize for SubscriberExposed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SubscriberExposed", 3)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("createdAt", &self.created_at.to_string())?;
        state.end()
    }
}
