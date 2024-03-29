#![allow(non_snake_case)]

use super::IncidentSeverity;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlmLineString {
    pub srsName: Option<String>,
    pub srsDimension: Option<String>,
    pub posList: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationLine {
    pub gmlLineString: GlmLineString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub locationDescriptor: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SituationRecord {
    pub startTime: String,
    pub endTime: Option<String>,
    pub id: String,
    pub severity: String,
    pub generalPublicComment: Option<String>,
    pub location: GlmLineString,
}

impl SituationRecord {
    pub fn get_severity(&self) -> Option<IncidentSeverity> {
        match IncidentSeverity::from_informal_severity(&self.severity) {
            Ok(it) => Some(it),
            Err(error) => {
                println!("failed to get incident severity: {:?}", error);
                None
            }
        }
    }
}
