use crate::model_prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ChallengeListing {
    pub challenges: Vec<String>,
}
