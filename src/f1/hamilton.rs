use anyhow::Result;
use serde::Deserialize;

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

pub fn fetch_car_data() -> Result<Vec<CarData>> {
    println!("Fetching data");
    let url = "https://api.openf1.org/v1/car_data?driver_number=44&session_key=latest";
    let data = reqwest::blocking::get(url)?.json::<Vec<CarData>>()?;
    println!("Finished fetching data");
    Ok(data)
}
