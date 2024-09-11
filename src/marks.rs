use crate::shared;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MarksResponse {
    #[serde(rename = "Subjects")]
    pub subjects: Vec<Subject>,
}

#[derive(Deserialize, Debug)]
pub struct Subject {
    #[serde(rename = "Marks")]
    pub marks: Vec<Mark>,
    #[serde(rename = "Subject")]
    pub subject: shared::Subject,
    #[serde(rename = "AverageText")]
    pub average_text: String,
    #[serde(rename = "TemporaryMark")]
    pub temporary_mark: String,
    #[serde(rename = "SubjectNote")]
    pub subject_note: String,
    #[serde(rename = "TemporaryMarkNote")]
    pub temporary_mark_note: String,
    #[serde(rename = "PointsOnly")]
    pub points_only: bool,
    #[serde(rename = "MarkPredictionEnabled")]
    pub mark_prediction_enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct Mark {
    #[serde(rename = "MarkDate")]
    pub mark_date: String,
    #[serde(rename = "EditDate")]
    pub edit_date: String,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "Theme")]
    pub theme: String,
    #[serde(rename = "MarkText")]
    pub mark_text: String,
    #[serde(rename = "TeacherId")]
    pub teacher_id: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "TypeNote")]
    pub type_note: String,
    #[serde(rename = "Weight")]
    pub weight: u8,
    #[serde(rename = "SubjectId")]
    pub subject_id: String,
    #[serde(rename = "IsNew")]
    pub is_new: bool,
    #[serde(rename = "IsPoints")]
    pub is_points: bool,
    #[serde(rename = "CalculatedMarkText")]
    pub calculated_mark_text: String,
    #[serde(rename = "ClassRankText")]
    pub class_rank_text: Option<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "PointsText")]
    pub points_text: String,
    #[serde(rename = "MaxPoints")]
    pub max_points: u16,
}

impl crate::BakalariClient {
    /// Get all marks in the current term.
    pub async fn get_marks(&mut self) -> Result<MarksResponse, crate::Error> {
        if self.check_if_token_expired() {
            self.refresh_login().await?;
        };

        let marks: MarksResponse = self
            .http_client
            .get(format!("{}/marks", &self.api_url))
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(marks)
    }

    /// Get the count of new, unread marks.
    pub async fn get_new_marks_count(&mut self) -> Result<u16, crate::Error> {
        if self.check_if_token_expired() {
            self.refresh_login().await?;
        };

        let count = self
            .http_client
            .get(format!("{}/marks/count-new", self.api_url))
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .text()
            .await?;

        Ok(count.parse()?)
    }
}

#[cfg(test)]
mod test {
    use crate::shared::test::get_credentials;
    use tokio_test::block_on;

    #[test]
    fn get_marks() -> Result<(), crate::Error> {
        let creds = get_credentials();
        let mut client = block_on(crate::BakalariClient::new(
            &creds.base_url,
            &creds.username,
            &creds.password,
        ))?;
        let marks = block_on(client.get_marks());

        println!("{marks:#?}");

        Ok(())
    }

    #[test]
    fn get_new_marks_count() -> Result<(), crate::Error> {
        let creds = get_credentials();
        let mut client = block_on(crate::BakalariClient::new(
            &creds.base_url,
            &creds.username,
            &creds.password,
        ))?;
        let count = block_on(client.get_new_marks_count())?;

        println!("{count:#?}");

        Ok(())
    }
}
