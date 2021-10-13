use pulldown_cmark::Options;
use serde::Deserialize;

use crate::options::Opt;

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

pub struct Constants {
    pub list_html: String,
    pub list_count: i64,
    pub opt: Opt,
    pub head_include: String,
    pub body_include: String,
    pub pulldown_cmark_options: Options,
    pub global_settings: Settings,
}