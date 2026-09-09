use crate::{
    http::{HttpMethod, HttpStatus, errors, executor::HttpExecutor, target::HttpTarget},
    tester::TestExecutor,
};
use httpmock::{HttpMockResponse, MockServer};
use std::string::ToString;
use std::{assert_matches, collections::HashMap};

#[tokio::test]
async fn test_execute_and_return_a_valid_result() {
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

    let result = executor.execute(&target).await;

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(200));
    assert!(value.latency > std::time::Duration::new(0, 0));
    assert_eq!(value.body, "OK");

    handle.assert();
}

#[tokio::test]
async fn test_headers_are_propagated() {
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
    let result = executor.execute(&target).await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(200));
    assert!(value.latency > std::time::Duration::new(0, 0));
    assert_eq!(value.body, "OK");

    handle.assert();
}

#[tokio::test]
async fn test_does_timeout_on_long_request() {
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

    let result = executor.execute(&target).await;

    assert!(result.is_err());
    let err = result.unwrap_err();

    assert_matches!(
        err.downcast_ref::<errors::ResponseError>(),
        Some(errors::ResponseError::Timeout(_))
    );
    handle.assert();
}

#[tokio::test]
async fn test_returns_error_on_non_200_status() {
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

    let result = executor.execute(&target).await;

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::ServerError(500));
    assert!(value.latency > std::time::Duration::new(0, 0));
    assert_eq!(value.body, "Internal Server Error");
    handle.assert();
}

#[tokio::test]
async fn test_post_request() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("POST").path("/create");
        then.status(201).body("Created");
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_post".to_string(),
        format!("{}/create", server.base_url()),
        crate::http::HttpMethod::POST,
        HashMap::new(),
        None,
        201.into(),
    );

    let result = executor.execute(&target).await;

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(201));
    assert_eq!(value.body, "Created");
    handle.assert();
}

#[tokio::test]
async fn test_put_request() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("PUT").path("/update/123");
        then.status(200).body("Updated");
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_put".to_string(),
        format!("{}/update/123", server.base_url()),
        crate::http::HttpMethod::PUT,
        HashMap::new(),
        None,
        200.into(),
    );

    let result = executor.execute(&target).await;

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(200));
    assert_eq!(value.body, "Updated");
    handle.assert();
}

#[tokio::test]
async fn test_delete_request() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("DELETE").path("/delete/123");
        then.status(204).body("");
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_delete".to_string(),
        format!("{}/delete/123", server.base_url()),
        crate::http::HttpMethod::DELETE,
        HashMap::new(),
        None,
        204.into(),
    );

    let result = executor.execute(&target).await;

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(204));
    assert_eq!(value.body, "");
    handle.assert();
}

#[tokio::test]
async fn test_post_with_body() {
    let server = MockServer::start();
    let handle = server.mock(|when, then| {
        when.method("POST").path("/api/data").body("test_body");
        then.status(200).body("Received: test_body");
    });
    let executor = HttpExecutor::default();
    let target = HttpTarget::new(
        "id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_post_body".to_string(),
        format!("{}/api/data", server.base_url()),
        crate::http::HttpMethod::POST,
        HashMap::new(),
        Some("test_body".to_string()),
        200.into(),
    );

    let result = executor.execute(&target).await;

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.status, HttpStatus::Success(200));
    assert_eq!(value.body, "Received: test_body");
    handle.assert();
}

#[tokio::test]
async fn test_all_methods_routing() {
    let server = MockServer::start();

    let get_handle = server.mock(|when, then| {
        when.method("GET").path("/get-endpoint");
        then.status(200).body("GET response");
    });

    let post_handle = server.mock(|when, then| {
        when.method("POST").path("/post-endpoint");
        then.status(201).body("POST response");
    });

    let put_handle = server.mock(|when, then| {
        when.method("PUT").path("/put-endpoint");
        then.status(200).body("PUT response");
    });

    let delete_handle = server.mock(|when, then| {
        when.method("DELETE").path("/delete-endpoint");
        then.status(204).body("");
    });

    let executor = HttpExecutor::default();

    let get_target = HttpTarget::new(
        "get_id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_get".to_string(),
        format!("{}/get-endpoint", server.base_url()),
        HttpMethod::GET,
        HashMap::new(),
        None,
        200.into(),
    );
    let get_result = executor.execute(&get_target).await.unwrap();
    assert_eq!(get_result.status, HttpStatus::Success(200));
    assert_eq!(get_result.body, "GET response");
    get_handle.assert();

    let post_target = HttpTarget::new(
        "post_id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_post".to_string(),
        format!("{}/post-endpoint", server.base_url()),
        HttpMethod::POST,
        HashMap::new(),
        None,
        201.into(),
    );
    let post_result = executor.execute(&post_target).await.unwrap();
    assert_eq!(post_result.status, HttpStatus::Success(201));
    assert_eq!(post_result.body, "POST response");
    post_handle.assert();

    let put_target = HttpTarget::new(
        "put_id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_put".to_string(),
        format!("{}/put-endpoint", server.base_url()),
        HttpMethod::PUT,
        HashMap::new(),
        None,
        200.into(),
    );
    let put_result = executor.execute(&put_target).await.unwrap();
    assert_eq!(put_result.status, HttpStatus::Success(200));
    assert_eq!(put_result.body, "PUT response");
    put_handle.assert();

    let delete_target = HttpTarget::new(
        "delete_id".to_string(),
        std::time::Duration::new(5, 0),
        1,
        "test_delete".to_string(),
        format!("{}/delete-endpoint", server.base_url()),
        HttpMethod::DELETE,
        HashMap::new(),
        None,
        204.into(),
    );
    let delete_result = executor.execute(&delete_target).await.unwrap();
    assert_eq!(delete_result.status, HttpStatus::Success(204));
    assert_eq!(delete_result.body, "");
    delete_handle.assert();
}
