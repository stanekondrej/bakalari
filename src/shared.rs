use serde::Deserialize;

/// Represents a subject
#[derive(Deserialize, Debug)]
pub struct Subject {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Abbrev")]
    pub abbreviation: String,
    #[serde(rename = "Name")]
    pub name: String,
}
