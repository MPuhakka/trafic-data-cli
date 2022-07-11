use super::{server_response::*, SituationRecord};

pub struct TraficService<'a> {
    api_url: &'a str,
    client: &'a reqwest::Client,
}

#[derive(Debug)]
pub enum TraficServiceError {
    Unexpected,
    Malformed,
}

impl<'a> TraficService<'a> {
    pub fn init(client: &'a reqwest::Client) -> TraficService<'a> {
        TraficService {
            api_url: "https://traffic-incidents.tampere.fi/api/v1",
            client,
        }
    }

    pub async fn get_incidents(&self) -> Result<Vec<SituationRecord>, TraficServiceError> {
        let res = self
            .client
            .get(self.api_url)
            .header("Accept", "application/json")
            .send()
            .await;

        if res.is_err() {
            return Err(TraficServiceError::Unexpected);
        }

        match res.unwrap().json::<ServerResponse>().await {
            Ok(res) => Ok(res.list_situations()),
            Err(reason) => {
                println!("{:?}", reason);
                Err(TraficServiceError::Malformed)
            }
        }
    }
}
