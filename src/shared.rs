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

#[cfg(test)]
pub mod test {
    pub struct Credentials {
        pub username: String,
        pub password: String,
        pub base_url: String,
    }
    pub fn get_credentials() -> Credentials {
        Credentials {
            username: std::env::var("BAKALARI_USERNAME").unwrap(),
            password: std::env::var("BAKALARI_PASSWORD").unwrap(),
            base_url: std::env::var("BAKALARI_BASE_URL").unwrap(),
        }
    }
}
