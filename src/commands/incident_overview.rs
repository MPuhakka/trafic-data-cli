use crate::trafic::*;
use std::collections::HashMap;

pub async fn incident_overview<'a>(trafic_service: &TraficService<'a>) {
    match trafic_service.get_incidents().await {
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
            println!("{:?}", reasons);
        }
        Err(reason) => println!("{:?}", reason),
    }
}
