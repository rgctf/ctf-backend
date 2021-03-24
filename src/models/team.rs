use butane::model;

#[model]
#[derive(Debug, Default, Clone)]
pub struct Team {
    #[auto]
    pub id: i64,
    pub name: String,
    pub token: String,
}
