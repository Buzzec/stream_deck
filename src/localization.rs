use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Localization {
    description: String,
    name: String,
    category: String,
    #[serde(flatten)]
    actions: HashMap<String, ActionLocalization>,
    localization: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ActionLocalization {
    name: String,
    tooltip: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Copy)]
pub enum LocalizationLanguage {
    #[serde(rename = "zh_CN")]
    Chinese,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "es")]
    Spanish,
}
