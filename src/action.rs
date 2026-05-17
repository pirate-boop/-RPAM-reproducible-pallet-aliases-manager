use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Action {
    pub name: String,
    pub category: String,
    pub exec: String,

    pub description: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub requires_sudo: bool,
}
