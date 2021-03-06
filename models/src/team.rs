use crate::model_prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TeamPrivateDetails {
    pub team: Team,
    pub token: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TeamRegisterRequest {
    pub name: String,
}
