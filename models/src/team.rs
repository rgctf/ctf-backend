use crate::model_prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub struct Team {
    pub id: u64,
    pub name: String,
}
