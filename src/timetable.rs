use chrono::{Datelike, Utc};
use serde::Deserialize;

/// The main timetable struct
#[derive(Deserialize, Debug)]
pub struct Timetable {
    #[serde(rename = "Hours")]
    pub hours: Vec<Hour>,
    #[serde(rename = "Days")]
    pub days: Vec<Day>,
    #[serde(rename = "Classes")]
    pub classes: Vec<Class>,
    #[serde(rename = "Groups")]
    pub groups: Vec<Group>,
    #[serde(rename = "Subjects")]
    pub subjects: Vec<Subject>,
    #[serde(rename = "Teachers")]
    pub teachers: Vec<Teacher>,
    #[serde(rename = "Rooms")]
    pub rooms: Vec<Room>,
    #[serde(rename = "Cycles")]
    pub cycles: Vec<Cycle>,
}

/// Represents a single hour in the timetable - most importantly its start and end
#[derive(Deserialize, Debug)]
pub struct Hour {
    #[serde(rename = "Id")]
    pub id: i8,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "BeginTime")]
    pub begin_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
}

/// Represents a day of the week
#[derive(Deserialize, Debug)]
pub struct Day {
    #[serde(rename = "Atoms")]
    pub atoms: Vec<Atom>,
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: i8,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "DayDescription")]
    pub day_description: String,
    #[serde(rename = "DayType")]
    pub day_type: String,
}

/// A timetable cell
#[derive(Deserialize, Debug)]
pub struct Atom {
    #[serde(rename = "HourId")]
    pub hour_id: i8,
    #[serde(rename = "GroupIds")]
    pub group_ids: Vec<String>,
    #[serde(rename = "SubjectId")]
    pub subject_id: Option<String>,
    #[serde(rename = "TeacherId")]
    pub teacher_id: Option<String>,
    #[serde(rename = "RoomId")]
    pub room_id: Option<String>,
    #[serde(rename = "CycleIds")]
    pub cycle_ids: Vec<String>,
    #[serde(rename = "Change")]
    pub change: Option<Change>,
    #[serde(rename = "HomeworkIds")]
    pub homework_ids: Vec<String>,
    #[serde(rename = "Theme")]
    pub theme: Option<String>,
}

/// Represents a change in the timetable, if any
#[derive(Deserialize, Debug)]
pub struct Change {
    #[serde(rename = "ChangeSubject")]
    pub change_subject: Option<String>,
    #[serde(rename = "Day")]
    pub day: String,
    #[serde(rename = "Hours")]
    pub hours: String,
    #[serde(rename = "ChangeType")]
    pub change_type: String, // TODO: this is an enum
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "TypeAbbrev")]
    pub type_abbreviation: Option<String>,
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
}

/// Represents a class
#[derive(Deserialize, Debug)]
pub struct Class {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Abbrev")]
    pub abbreviation: String,
    #[serde(rename = "Name")]
    pub name: String,
}

/// Represents a group
#[derive(Deserialize, Debug)]
pub struct Group {
    #[serde(rename = "ClassId")]
    pub class_id: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Abbrev")]
    pub abbreviation: String,
    #[serde(rename = "Name")]
    pub name: String,
}

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

/// Represents a teacher
#[derive(Deserialize, Debug)]
pub struct Teacher {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Abbrev")]
    pub abbreviation: String,
    #[serde(rename = "Name")]
    pub name: String,
}

/// Represents a physical room (and rarely a mere location, e.g. "outside")
#[derive(Deserialize, Debug)]
pub struct Room {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Abbrev")]
    pub abbreviation: String,
    #[serde(rename = "Name")]
    pub name: String, // this is usually an empty string
}

/// A rotation cycle - symbolizes the fact that each odd week a student might have Biology on
/// Friday, whereas on even weeks, Friday Biology gets replaced by Maths
#[derive(Deserialize, Debug)]
pub struct Cycle {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Abbrev")]
    pub abbreviation: String,
    #[serde(rename = "Name")]
    pub name: String,
}

impl crate::BakalariClient {
    /// Get timetable by date
    pub async fn get_timetable(
        &mut self,
        date: chrono::DateTime<Utc>,
    ) -> Result<Timetable, reqwest::Error> {
        if self.check_if_token_expired() {
            self.refresh_login().await?;
        }

        let day: String;
        if date.day() < 10 {
            day = String::from(format!("0{}", date.day()))
        } else {
            day = date.day().to_string()
        }

        let month: String;
        if date.month() < 10 {
            month = String::from(format!("0{}", date.month()))
        } else {
            month = date.month().to_string()
        }

        // yyyy-mm-dd
        let date_string = format!("{}-{}-{}", date.year(), month, day);

        let timetable = self
            .http_client
            .get(format!(
                "{}/timetable/actual?date={date_string}",
                &self.api_url
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(timetable)
    }

    /// Convenience function to get current timetable
    pub async fn get_current_timetable(&mut self) -> Result<Timetable, reqwest::Error> {
        self.get_timetable(chrono::offset::Utc::now()).await
    }

    /// Get the permanent timetable
    pub async fn get_permanent_timetable(&mut self) -> Result<Timetable, reqwest::Error> {
        if self.check_if_token_expired() {
            self.refresh_login().await?;
        };

        let timetable: Timetable = self
            .http_client
            .get(format!("{}/timetable/permanent", &self.api_url))
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(timetable)
    }
}
