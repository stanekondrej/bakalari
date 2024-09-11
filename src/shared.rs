//! This module contains shared code for the different modules.

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
    use tokio_test::block_on;

    pub fn setup_client() -> Result<crate::BakalariClient, crate::Error> {
        let username = std::env::var("BAKALARI_USERNAME").unwrap();
        let password = std::env::var("BAKALARI_PASSWORD").unwrap();
        let base_url = std::env::var("BAKALARI_BASE_URL").unwrap();

        block_on(crate::BakalariClient::new(&base_url, &username, &password))
    }
}
