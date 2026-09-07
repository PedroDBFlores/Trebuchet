use super::ResponseError;

#[test]
fn test_response_error_display() {
    let error = ResponseError::Timeout(500);
    assert_eq!(error.to_string(), "Timeout after 500ms");

    let error = ResponseError::ConnectionError(std::io::Error::other("your message here"));
    assert_eq!(error.to_string(), "Connection error: your message here");
}
