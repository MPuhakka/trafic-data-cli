#![allow(non_snake_case)]
use super::situation_record::SituationRecord;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
struct SituationPublicationLight {
    situationRecord: Vec<SituationRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncidentResponse {
    modelBaseVersion: String,
    situationPublicationLight: SituationPublicationLight,
}

impl IncidentResponse {
    pub fn list_situations(&self) -> Vec<SituationRecord> {
        self.situationPublicationLight.situationRecord.to_owned()
    }
}
