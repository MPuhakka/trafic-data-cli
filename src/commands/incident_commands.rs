use crate::{
    data_models::{IncidentSeverity, SituationRecord},
    services::*,
    utilities::TablePrinter,
};
use clap::Args;
use std::collections::HashMap;

#[derive(Args)]
pub struct TraficIncidentList {
    #[clap(value_parser)]
    severity: Option<String>,
}

#[derive(Args)]
pub struct TraficOverview {}

pub struct IncidentCommandHandler<'a> {
    pub trafic_service: &'a TraficService<'a>,
    pub table_printer: &'a TablePrinter,
}

impl<'a> IncidentCommandHandler<'a> {
    pub async fn incident_overview(&self) {
        match self.trafic_service.get_incidents().await {
            Ok(incidents) => {
                let mut reasons: HashMap<String, u16> = HashMap::new();
                for incident in incidents.iter() {
                    let key = &incident.severity;
                    let value: u16 = if reasons.contains_key(key) {
                        reasons.get(key).unwrap() + 1
                    } else {
                        1
                    };
                    reasons.insert(key.clone(), value);
                }

                let mut print_data: Vec<Vec<String>> = Vec::new();
                let header_row: Vec<String> = vec!["informal severity", "severity", "count"]
                    .iter()
                    .map(|it| it.to_uppercase().to_string())
                    .collect();
                print_data.push(header_row);
                for (key, value) in reasons.iter() {
                    let severity = match IncidentSeverity::from_informal_severity(key) {
                        Ok(it) => it.to_string(),
                        Err(_) => String::from(" - "),
                    };

                    let row: Vec<String> = vec![String::from(key), severity, value.to_string()];
                    print_data.push(row);
                }
                self.table_printer.print(&print_data);
            }
            Err(reason) => println!("{:?}", reason),
        }
    }

    pub async fn list_incidents(&self, command: &TraficIncidentList) {
        match self.trafic_service.get_incidents().await {
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

                let mut data: Vec<Vec<String>> = Vec::new();
                let header_row: Vec<String> = vec!["id", "start time", "end time", "severity"]
                    .iter()
                    .map(|it| it.to_uppercase().to_string())
                    .collect();
                data.push(header_row);

                for incident in filtered.iter() {
                    let severity_string = match incident.get_severity() {
                        Some(it) => it.to_string(),
                        None => String::from(" - "),
                    };
                    data.push(vec![
                        incident.id.to_owned(),
                        incident.startTime.to_owned(),
                        incident
                            .endTime
                            .as_ref()
                            .unwrap_or(&String::from("none"))
                            .to_owned(),
                        severity_string,
                    ]);
                }

                self.table_printer.print(&data);
            }
            Err(reason) => println!("{:?}", reason),
        }
    }
}
