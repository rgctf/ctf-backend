use butane::model;

#[model]
#[derive(Debug, Default, Clone)]
pub struct Team {
    #[auto]
    pub id: i64,
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
        let token = self.token.clone();
        backend_models::TeamPrivateDetails {
            team: self.into(),
            token,
        }
    }
}
