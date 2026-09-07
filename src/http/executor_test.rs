use crate::{
    http::{HttpStatus, errors, executor::HttpExecutor, target::HttpTarget},
    tester::TestExecutor,
};
use httpmock::{HttpMockResponse, MockServer};
use std::string::ToString;
use std::{assert_matches, collections::HashMap};

#[test]
fn test_execute_and_return_a_valid_result() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("GET").path("/");
        then.status(200).body("OK");
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test".to_string(),
        server.base_url(),
        crate::http::HttpMethod::GET,
        HashMap::new(),
        None,
        200.into(),
    );

    let result = executor.execute(&target);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(200));
    assert!(value.latency > std::time::Duration::new(0, 0));
    assert_eq!(value.body, "OK");

    handle.assert();
}

#[test]
fn test_headers_are_propagated() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("GET").path("/").header("a_nice", "header");
        then.status(200).body("OK");
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test".to_string(),
        server.base_url(),
        crate::http::HttpMethod::GET,
        HashMap::from([("a_nice".to_string(), "header".to_string())]),
        None,
        200.into(),
    );
    let result = executor.execute(&target);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(200));
    assert!(value.latency > std::time::Duration::new(0, 0));
    assert_eq!(value.body, "OK");

    handle.assert();
}

#[test]
fn test_does_timeout_on_long_request() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("GET").path("/");
        then.respond_with(|_| {
            std::thread::sleep(std::time::Duration::new(2, 0));
            HttpMockResponse::builder().status(200).body("OK").build()
        });
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(1, 0),
        1,
        "test".to_string(),
        server.base_url(),
        crate::http::HttpMethod::GET,
        HashMap::new(),
        None,
        200.into(),
    );
    let result = executor.execute(&target);
    assert!(result.is_err());
    let err = result.unwrap_err();

    assert_matches!(
        err.downcast_ref::<errors::ResponseError>(),
        Some(errors::ResponseError::Timeout(_))
    );
    handle.assert();
}

#[test]
fn test_returns_error_on_non_200_status() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("GET").path("/");
        then.status(500).body("Internal Server Error");
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test".to_string(),
        server.base_url(),
        crate::http::HttpMethod::GET,
        HashMap::new(),
        None,
        200.into(),
    );

    let result = executor.execute(&target);

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::ServerError(500));
    assert!(value.latency > std::time::Duration::new(0, 0));
    assert_eq!(value.body, "Internal Server Error");
    handle.assert();
}
