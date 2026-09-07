use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum HttpStatus {
    Success(u16),
    Informational(u16),
    Redirection(u16),
    ClientError(u16),
    ServerError(u16),
    Unknown(u16),
}

impl From<u16> for HttpStatus {
    fn from(code: u16) -> Self {
        match code {
            100..=199 => HttpStatus::Informational(code),
            200..=299 => HttpStatus::Success(code),
            300..=399 => HttpStatus::Redirection(code),
            400..=499 => HttpStatus::ClientError(code),
            500..=599 => HttpStatus::ServerError(code),
            _ => HttpStatus::Unknown(code),
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            HttpStatus::Informational(val) => format!("Informational ({})", val),
            HttpStatus::Success(val) => format!("Success ({})", val),
            HttpStatus::Redirection(val) => format!("Redirect ({})", val),
            HttpStatus::ClientError(val) => format!("Client Error ({})", val),
            HttpStatus::ServerError(val) => format!("Server Error ({})", val),
            HttpStatus::Unknown(val) => format!("Unknown ({})", val),
        };
        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status_from_u16() {
        assert_eq!(HttpStatus::from(101), HttpStatus::Informational(101));
        assert_eq!(HttpStatus::from(200), HttpStatus::Success(200));
        assert_eq!(HttpStatus::from(301), HttpStatus::Redirection(301));
        assert_eq!(HttpStatus::from(404), HttpStatus::ClientError(404));
        assert_eq!(HttpStatus::from(500), HttpStatus::ServerError(500));
        assert_eq!(HttpStatus::from(600), HttpStatus::Unknown(600));
    }

    #[test]
    fn test_http_status_description() {
        assert_eq!(HttpStatus::from(101).to_string(), "Informational (101)");
        assert_eq!(HttpStatus::from(200).to_string(), "Success (200)");
        assert_eq!(HttpStatus::from(301).to_string(), "Redirect (301)");
        assert_eq!(HttpStatus::from(404).to_string(), "Client Error (404)");
        assert_eq!(HttpStatus::from(500).to_string(), "Server Error (500)");
        assert_eq!(HttpStatus::from(600).to_string(), "Unknown (600)");
    }
}
