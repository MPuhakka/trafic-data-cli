use crate::services::*;

pub async fn list_incidents<'a>(service: &TraficService<'a>) {
    match service.get_incidents().await {
        Ok(incidents) => {
            for incident in incidents.iter() {
                println!("{}", &incident.toDisplayString());
            }
        }
        Err(reason) => println!("{:?}", reason),
    }
}
