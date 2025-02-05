use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BeamAppPost {
    pub beam_id: BeamAppName,
    pub beam_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct BeamAppDelete {
    pub beam_id: BeamAppName,
}

#[derive(Serialize)]
pub struct BeamAppName(String);

impl std::ops::Deref for BeamAppName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for BeamAppName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        <String>::deserialize(deserializer).and_then(|v| if re.is_match(&v) {
            Ok(v)
        } else {
            Err(serde::de::Error::custom("Value does not match ^[a-zA-Z0-9]+$"))
        }).map(Self)
    }
}