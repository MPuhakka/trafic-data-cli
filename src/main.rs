mod trafic;
use std::collections::HashMap;

use crate::trafic::*;

#[tokio::main]
async fn main() {
    let client = &reqwest::Client::new();
    let trafic_service = TraficService::init(client);
    match trafic_service.get_incidents().await {
        Ok(incidents) => {
            println!("received {} incidents", incidents.len());
            let mut reasons: HashMap<String, u16> = HashMap::new();
            for incident in incidents.iter() {
                // println!("{}", &incident.toDisplayString());
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
