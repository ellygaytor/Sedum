use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Page {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub list: Option<String>,
}