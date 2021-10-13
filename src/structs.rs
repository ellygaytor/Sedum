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

#[derive(Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "default_author")]
    pub default_author: String,
}

fn default_author() -> String {
    "Sedum".to_string()
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_author: ("Sedum").to_string(),
        }
    }
}
