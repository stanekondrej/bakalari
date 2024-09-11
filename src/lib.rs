mod timetable;

#[cfg(test)]
mod test;

use serde::Deserialize;

/// The actual client. Use its associated methods to access the actual data.
#[derive(Debug)]
pub struct BakalariClient {
    access_token: String,
    refresh_token: String,
    http_client: reqwest::Client,
    base_url: String
}

impl BakalariClient {
    /// Construct a new BakalariClient. The URL should be without any trailing slashes,
    /// e.g. in the following format: `https://bakalari.school.tld`
    pub async fn new(base_url: &str, username: &str, password: &str) -> Result<Self, reqwest::Error> {
        #[derive(Deserialize)]
        struct Response {
            access_token: String,
            refresh_token: String,
        }

        let client = reqwest::Client::new();
        let response: Response = client
            .post(format!("{base_url}/api/login"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("client_id=ANDR&grant_type=password&username={username}&password={password}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(BakalariClient {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            http_client: client,
            base_url: format!("{}/api/3", base_url.to_string())
        })
    }


}
