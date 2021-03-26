use chrono::NaiveDateTime;

use crate::model_prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Event {
    pub id: String,
    pub name: String,
    pub start_timestamp: Option<NaiveDateTime>,
    pub end_timestamp: Option<NaiveDateTime>,
}
