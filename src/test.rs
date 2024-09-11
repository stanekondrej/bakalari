use tokio_test::block_on;

use crate::BakalariClient;

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

#[test]
fn login() -> Result<(), reqwest::Error> {
    let creds = get_credentials();
    let client = block_on(BakalariClient::new(
        &creds.base_url,
        &creds.username,
        &creds.password,
    ))?;

    println!("{client:#?}");

    Ok(())
}

#[test]
fn get_timetable() -> Result<(), reqwest::Error> {
    let creds = get_credentials();
    let mut client = block_on(BakalariClient::new(
        &creds.base_url,
        &creds.username,
        &creds.password,
    ))?;
    let timetable = block_on(client.get_timetable(chrono::offset::Utc::now()))?;

    println!("{timetable:#?}");

    Ok(())
}

#[test]
fn get_current_timetable() -> Result<(), reqwest::Error> {
    let creds = get_credentials();
    let mut client = block_on(BakalariClient::new(
        &creds.base_url,
        &creds.username,
        &creds.password,
    ))?;
    let timetable = block_on(client.get_current_timetable())?;

    println!("{timetable:#?}");

    Ok(())
}

#[test]
fn get_permanent_timetable() -> Result<(), reqwest::Error> {
    let creds = get_credentials();
    let mut client = block_on(BakalariClient::new(
        &creds.base_url,
        &creds.username,
        &creds.password,
    ))?;
    let timetable = block_on(client.get_permanent_timetable())?;

    println!("{timetable:#?}");

    Ok(())
}
