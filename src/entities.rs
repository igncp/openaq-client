#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub count: u32,
    pub country: String,
    pub locations: u32,
    #[serde(rename = "city")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub cities: u32,
    pub code: String,
    pub count: u32,
    pub locations: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchResult {
    pub message: String,
    pub count: u32,
    pub duration: f32,
    #[serde(rename = "sourceName")]
    pub source_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fetch {
    #[serde(rename = "timeStarted")]
    pub time_started: String,
    #[serde(rename = "timeEnded")]
    pub time_ended: String,
    pub count: u32,
    pub results: Vec<FetchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "preferredUnit")]
    pub preferred_unit: String,
}
