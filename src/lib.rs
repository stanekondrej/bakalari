mod timetable;

#[cfg(test)]
mod test;

use serde::Deserialize;

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
    pub async fn new(
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let response: LoginResponse = client
            .post(format!("{base_url}/api/login")) // this is really weird - /api/3/login doesn't exist
            .header("Content-Type", "application/x-www-form-urlencoded")
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

    async fn refresh_login(&mut self) -> Result<(), reqwest::Error> {
        let response: LoginResponse = self
            .http_client
            .post(format!("{}/api/login", self.base_url))
            .header("Content-Type", "application/x-www-form-urlencoded")
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
