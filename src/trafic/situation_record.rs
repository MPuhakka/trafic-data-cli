#![allow(non_snake_case)]

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
    pub fn toDisplayString(&self) -> String {
        let comment = match &self.generalPublicComment {
            Some(it) => it,
            None => "no additional information",
        };

        let end = match &self.endTime {
            Some(it) => it,
            None => "ongoing",
        };

        format!(
            "{}\n{}\n{}\n{}\n{}\n",
            self.id, self.startTime, end, self.severity, comment
        )
    }
}
