use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Driver {
    pub driver_number: u32,
    pub full_name: String,
    pub name_acronym: String,
    pub team_name: Option<String>,
    pub team_colour: Option<String>,
    pub headshot_url: Option<String>,
}

pub fn fetch_drivers(session_key: u32) -> Vec<Driver> {
    let url = format!("https://api.openf1.org/v1/drivers?session_key={session_key}");
    let bytes = match reqwest::blocking::get(&url).and_then(|r| r.bytes()) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error fetching drivers: {e:?}");
            return vec![];
        }
    };
    let mut drivers: Vec<Driver> = serde_json::from_slice(&bytes).unwrap_or_default();
    drivers.sort_by_key(|d| d.driver_number);
    drivers
}
