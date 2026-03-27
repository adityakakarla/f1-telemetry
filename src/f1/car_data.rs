use anyhow::Result;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize, Clone)]
pub struct CarData {
    pub brake: Option<u32>,
    pub date: String,
    pub driver_number: Option<u32>,
    pub drs: Option<u32>,
    pub meeting_key: Option<u32>,
    pub n_gear: Option<u32>,
    pub rpm: Option<u32>,
    pub session_key: Option<u32>,
    pub speed: Option<u32>,
    pub throttle: Option<u32>,
}

pub fn fetch_car_data(driver_number: u32, session_key: u32) -> Result<Vec<CarData>> {
    println!("Fetching data for driver {driver_number}, session {session_key}");
    let url = format!(
        "https://api.openf1.org/v1/car_data?driver_number={driver_number}&session_key={session_key}"
    );
    let bytes = reqwest::blocking::get(&url)?.bytes()?;
    let data = serde_json::from_slice::<Vec<CarData>>(&bytes).unwrap_or_default();
    println!("Finished fetching data ({} points)", data.len());
    Ok(data)
}
