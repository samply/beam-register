use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct BeamApp {
    pub beam_id: Option<String>,
    pub beam_secret: Option<String>,
}

#[derive(Debug)]
pub enum BeamAppError {
    MissingBeamId,
    MissingBeamSecret,
    InvalidBeamIdFormat,
}

impl BeamApp {

    // Validate if the beam_id is valid (only alphanumeric, underscores, and dashes)
    pub fn is_valid_beam_id(&self) -> Result<(), BeamAppError> {
        match &self.beam_id {
            Some(id) => {
                let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
                if re.is_match(id) {
                    Ok(())
                } else {
                    Err(BeamAppError::InvalidBeamIdFormat)
                }
            }
            None => Err(BeamAppError::MissingBeamId),
        }
    }

    // Validate both beam_id and beam_secret
    pub fn validate(&self) -> Result<(), BeamAppError> {
        self.is_valid_beam_id()?;
        match &self.beam_secret {
            Some(secret) if !secret.is_empty() => Ok(()),
            Some(_) => Err(BeamAppError::MissingBeamSecret),
            None => Err(BeamAppError::MissingBeamSecret),
        }
    }

}
