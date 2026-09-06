#[derive(Debug, Clone, PartialEq)]
pub(crate) enum HttpMethod {
    GET = 0,
    POST = 1,
    PUT = 2,
    DELETE = 3,
}

impl From<HttpMethod> for String {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
        }
    }
}

impl TryFrom<&str> for HttpMethod {
    type Error = String;

    fn try_from(method: &str) -> Result<Self, Self::Error> {
        match method {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(format!("invalid HTTPMethod: {}", method)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HttpMethod;

    #[test]
    fn test_string_from_http_method() {
        let get: String = HttpMethod::GET.into();
        assert_eq!(get, "GET");
        let post: String = HttpMethod::POST.into();
        assert_eq!(post, "POST");
        let put: String = HttpMethod::PUT.into();
        assert_eq!(put, "PUT");
        let delete: String = HttpMethod::DELETE.into();
        assert_eq!(delete, "DELETE");
    }

    #[test]
    fn test_http_method_from_string() {
        let get: Result<HttpMethod, String> = "GET".try_into();
        assert!(get.is_ok());
        assert_eq!(get.unwrap(), HttpMethod::GET);
        let post: Result<HttpMethod, String> = "POST".try_into();
        assert!(post.is_ok());
        assert_eq!(post.unwrap(), HttpMethod::POST);
        let put: Result<HttpMethod, String> = "PUT".try_into();
        assert!(put.is_ok());
        assert_eq!(put.unwrap(), HttpMethod::PUT);
        let delete: Result<HttpMethod, String> = "DELETE".try_into();
        assert!(delete.is_ok());
        assert_eq!(delete.unwrap(), HttpMethod::DELETE);
        let invalid: Result<HttpMethod, String> = "INVALID".try_into();
        assert!(invalid.is_err());
        assert_eq!(invalid.unwrap_err(), "invalid HTTPMethod: INVALID");
    }
}
