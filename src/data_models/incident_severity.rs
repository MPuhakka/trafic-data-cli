use super::*;

#[derive(PartialEq)]
pub enum IncidentSeverity {
    High,
    Low,
}

impl IncidentSeverity {
    pub fn from_informal_severity(severity: &String) -> Result<IncidentSeverity, InvalidDataError> {
        match severity.as_str() {
            "Haittaa liikenteelle" => Ok(IncidentSeverity::Low),
            "Merkittävää haittaa liikenteelle" => Ok(IncidentSeverity::High),
            _ => Err(InvalidDataError::new(ErrorReason::Is(format!(
                "unknown informal data severity: {}",
                severity
            )))),
        }
    }

    pub fn from_string(severity: &String) -> Result<IncidentSeverity, InvalidDataError> {
        match severity.to_lowercase().as_str() {
            "high" => Ok(IncidentSeverity::High),
            "low" => Ok(IncidentSeverity::Low),
            _ => Err(InvalidDataError::new(ErrorReason::Is(format!(
                "unknown data severity {}",
                severity
            )))),
        }
    }
}

impl ToString for IncidentSeverity {
    fn to_string(&self) -> String {
        match &self {
            IncidentSeverity::High => String::from("high"),
            IncidentSeverity::Low => String::from("low"),
        }
    }
}
