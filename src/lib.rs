//! # bakalari
//!
//! The `bakalari` crate aims to provide a layer on top of the Bakaláři API, so that using it is
//! easier and safer. The API isn't officially documented, it changes at random and without notice.
//! By using something like the `bakalari` crate, you eliminate the need to worry about these
//! changes yourself.
//!
//! Built on top of `reqwest`, this crate is already pretty fast. It has an easily understandable
//! async interface, and I try my best to document the struct fields as well.
//!
//! When building this library, I closely followed the specification in
//! [this repo](https://github.com/bakalari-api/bakalari-api-v3). It has been a huge help and I
//! thank all of the contributors for their effort.

mod marks;
mod shared;
mod timetable;

use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error with http request")]
    Reqwest(#[from] reqwest::Error),
    #[error("unable to parse a number")]
    ParseError(#[from] std::num::ParseIntError),
}

/// The actual client. Use its associated methods to access the actual data.
#[derive(Debug)]
pub struct BakalariClient {
    access_token: String,
    token_expires: chrono::DateTime<chrono::Utc>,
    refresh_token: String,
    http_client: reqwest::Client,
    base_url: String,
    api_url: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

impl BakalariClient {
    /// Construct a new BakalariClient. The URL should be without any trailing slashes,
    /// e.g. in the following format: `https://bakalari.school.tld`
    pub async fn new(base_url: &str, username: &str, password: &str) -> Result<Self, crate::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let response: LoginResponse = client
            .post(format!("{base_url}/api/login")) // this is really weird - /api/3/login doesn't exist
            .body(format!(
                "client_id=ANDR&grant_type=password&username={username}&password={password}"
            ))
            .send()
            .await?
            .json()
            .await?;

        let token_expires =
            chrono::offset::Utc::now() + chrono::Duration::seconds(response.expires_in);

        Ok(BakalariClient {
            access_token: response.access_token,
            token_expires,
            refresh_token: response.refresh_token,
            http_client: client,
            base_url: base_url.to_string(),
            api_url: format!("{}/api/3", base_url.to_string()),
        })
    }

    async fn refresh_login(&mut self) -> Result<(), crate::Error> {
        let response: LoginResponse = self
            .http_client
            .post(format!("{}/api/login", self.base_url))
            .body(format!(
                "client_id=ANDR=grant_type=refresh_token&refresh_token={}",
                self.refresh_token
            ))
            .send()
            .await?
            .json()
            .await?;

        self.access_token = response.access_token;
        self.refresh_token = response.refresh_token;

        Ok(())
    }

    fn check_if_token_expired(&self) -> bool {
        chrono::offset::Utc::now() > self.token_expires
    }
}

#[cfg(test)]
mod test {
    use crate::shared::test::setup_client;

    #[test]
    fn login() -> Result<(), crate::Error> {
        let client = setup_client()?;

        println!("{client:#?}");

        Ok(())
    }
}
