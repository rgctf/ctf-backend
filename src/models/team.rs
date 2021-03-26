use butane::model;

#[model]
#[derive(Debug, Default)]
pub struct Team {
    #[auto]
    pub id: i32,
    pub name: String,
    pub token: String,
}

impl Into<backend_models::Team> for Team {
    fn into(self) -> backend_models::Team {
        backend_models::Team {
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<backend_models::TeamPrivateDetails> for Team {
    fn into(self) -> backend_models::TeamPrivateDetails {
        backend_models::TeamPrivateDetails {
            token: self.token.clone(),
            team: self.into(),
        }
    }
}
