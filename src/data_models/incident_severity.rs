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
