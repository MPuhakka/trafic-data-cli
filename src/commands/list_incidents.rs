use crate::{
    data_models::{IncidentSeverity, SituationRecord},
    services::*,
};
use clap::Args;

#[derive(Args)]
pub struct TraficIncidentList {
    #[clap(value_parser)]
    severity: Option<String>,
}

pub async fn list_incidents<'a>(service: &TraficService<'a>, command: &TraficIncidentList) {
    match service.get_incidents().await {
        Ok(incidents) => {
            let requested_severity = match &command.severity {
                Some(it) => match IncidentSeverity::from_string(it) {
                    Ok(severity) => Some(severity),
                    Err(error) => {
                        panic!("list incidents failed: {}", error);
                    }
                },
                None => None,
            };

            let owned_incidetes = incidents.to_vec();
            let filtered: Vec<SituationRecord> = match requested_severity {
                Some(severity) => owned_incidetes
                    .into_iter()
                    .filter(|item| match item.get_severity() {
                        Some(it) => it == severity,
                        None => false,
                    })
                    .collect::<Vec<SituationRecord>>(),

                None => incidents.to_vec(),
            };

            for incident in filtered.iter() {
                println!("{}", &incident.to_display_string());
            }
        }
        Err(reason) => println!("{:?}", reason),
    }
}
