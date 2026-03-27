use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
    pub session_key: u32,
    pub session_name: String,
    pub location: String,
    pub country_name: String,
    pub date_start: String,
    pub year: u32,
}

pub fn fetch_sessions(year: u32) -> Vec<Session> {
    let url = format!("https://api.openf1.org/v1/sessions?year={year}");
    let bytes = match reqwest::blocking::get(&url).and_then(|r| r.bytes()) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error fetching sessions: {e:?}");
            return vec![];
        }
    };
    let mut sessions: Vec<Session> = serde_json::from_slice(&bytes).unwrap_or_default();
    sessions.sort_by(|a, b| a.date_start.cmp(&b.date_start));
    sessions
}
