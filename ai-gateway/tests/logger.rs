use std::collections::HashMap;

use ai_gateway::{
    config::{
        Config, deployment_target::DeploymentTarget, helicone::HeliconeFeatures,
    },
    tests::{TestDefault, harness::Harness, mock::MockArgs},
};
use http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use tower::Service;

#[tokio::test]
#[serial_test::serial]
async fn request_response_logger_authenticated() {
    let mut config = Config::test_default();
    // Ensure auth is required for this test
    config.helicone.features = HeliconeFeatures::All;

    let mock_args = MockArgs::builder()
        .stubs(HashMap::from([
            ("success:openai:chat_completion", 1.into()),
            ("success:minio:upload_request", 1.into()),
            ("success:jawn:log_request", 1.into()),
            ("success:jawn:sign_s3_url", 1.into()),
        ]))
        .build();
    let mut harness = Harness::builder()
        .with_config(config)
        .with_mock_args(mock_args)
        .with_mock_auth()
        .build()
        .await;
    let body_bytes = serde_json::to_vec(&json!({
        "model": "openai/gpt-4o-mini",
        "messages": [
            {
                "role": "user",
                "content": "Hello, world!"
            }
        ]
    }))
    .unwrap();

    let request_body = axum_core::body::Body::from(body_bytes.clone());
    let request = Request::builder()
        .method(Method::POST)
        .header("authorization", "Bearer sk-helicone-test-key")
        .uri("http://router.helicone.com/router/my-router/chat/completions")
        .body(request_body)
        .unwrap();
    let response = harness.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // we need to collect the body here in order to poll the underlying body
    // so that the async logging task can complete
    let _response_body = response.into_body().collect().await.unwrap();

    // sleep so that the background task for logging can complete
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
}

#[tokio::test]
#[serial_test::serial]
async fn authenticated_sidecar() {
    let mut config = Config::test_default();
    let minio_port = 9190;
    // Ensure auth is required for this test
    config.helicone.features = HeliconeFeatures::All;
    config.deployment_target = DeploymentTarget::Sidecar;

    let mock_args = MockArgs::builder()
        .stubs(HashMap::from([
            ("success:openai:chat_completion", 1.into()),
            ("success:minio:upload_request", 1.into()),
            ("success:jawn:log_request", 1.into()),
            ("success:jawn:sign_s3_url", 1.into()),
        ]))
        .minio_port(minio_port)
        .build();
    let mut harness = Harness::builder()
        .with_config(config)
        .with_mock_args(mock_args)
        .with_mock_auth()
        .build()
        .await;
    let body_bytes = serde_json::to_vec(&json!({
        "model": "openai/gpt-4o-mini",
        "messages": [
            {
                "role": "user",
                "content": "Hello, world!"
            }
        ]
    }))
    .unwrap();

    let request_body = axum_core::body::Body::from(body_bytes.clone());
    let request = Request::builder()
        .method(Method::POST)
        .header("authorization", "Bearer sk-helicone-test-key")
        .uri("http://router.helicone.com/router/my-router/chat/completions")
        .body(request_body)
        .unwrap();
    let response = harness.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // we need to collect the body here in order to poll the underlying body
    // so that the async logging task can complete
    let _response_body = response.into_body().collect().await.unwrap();

    // sleep so that the background task for logging can complete
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
}

#[tokio::test]
#[serial_test::serial]
async fn unauthenticated_sidecar() {
    let mut config = Config::test_default();
    let minio_port = 9190;
    // Ensure auth is required for this test
    config.helicone.features = HeliconeFeatures::Auth;
    config.deployment_target = DeploymentTarget::Sidecar;

    let mock_args = MockArgs::builder()
        .stubs(HashMap::from([
            ("success:openai:chat_completion", 0.into()),
            ("success:minio:upload_request", 0.into()),
            ("success:jawn:log_request", 0.into()),
            ("success:jawn:sign_s3_url", 0.into()),
        ]))
        .minio_port(minio_port)
        .build();
    let mut harness = Harness::builder()
        .with_config(config)
        .with_mock_args(mock_args)
        .with_mock_auth()
        .build()
        .await;
    let body_bytes = serde_json::to_vec(&json!({
        "model": "openai/gpt-4o-mini",
        "messages": [
            {
                "role": "user",
                "content": "Hello, world!"
            }
        ]
    }))
    .unwrap();

    let request_body = axum_core::body::Body::from(body_bytes.clone());
    let request = Request::builder()
        .method(Method::POST)
        .uri("http://router.helicone.com/router/my-router/chat/completions")
        .body(request_body)
        .unwrap();
    let response = harness.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial_test::serial]
async fn request_response_logger_unauthenticated() {
    let mut config = Config::test_default();
    // Disable auth requirement for this test
    config.helicone.features = HeliconeFeatures::None;

    let mock_args = MockArgs::builder()
        .stubs(HashMap::from([
            ("success:openai:chat_completion", 1.into()),
            // When unauthenticated, logging services should NOT be called
            ("success:minio:upload_request", 0.into()),
            ("success:jawn:log_request", 0.into()),
            ("success:jawn:sign_s3_url", 0.into()),
        ]))
        .build();
    let mut harness = Harness::builder()
        .with_config(config)
        .with_mock_args(mock_args)
        .build()
        .await;
    let body_bytes = serde_json::to_vec(&json!({
        "model": "openai/gpt-4o-mini",
        "messages": [
            {
                "role": "user",
                "content": "Hello, world!"
            }
        ]
    }))
    .unwrap();

    let request_body = axum_core::body::Body::from(body_bytes.clone());
    let request = Request::builder()
        .method(Method::POST)
        // No authorization header when auth is not required
        .uri("http://router.helicone.com/router/my-router/chat/completions")
        .body(request_body)
        .unwrap();
    let response = harness.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[serial_test::serial]
async fn request_response_logger_unauthenticated_sidecar() {
    let mut config = Config::test_default();
    // Disable auth requirement for this test
    config.helicone.features = HeliconeFeatures::None;
    config.deployment_target = DeploymentTarget::Sidecar;

    let mock_args = MockArgs::builder()
        .stubs(HashMap::from([
            ("success:openai:chat_completion", 1.into()),
            // When unauthenticated, logging services should NOT be called
            ("success:minio:upload_request", 0.into()),
            ("success:jawn:log_request", 0.into()),
            ("success:jawn:sign_s3_url", 0.into()),
        ]))
        .build();
    let mut harness = Harness::builder()
        .with_config(config)
        .with_mock_args(mock_args)
        .build()
        .await;
    let body_bytes = serde_json::to_vec(&json!({
        "model": "openai/gpt-4o-mini",
        "messages": [
            {
                "role": "user",
                "content": "Hello, world!"
            }
        ]
    }))
    .unwrap();

    let request_body = axum_core::body::Body::from(body_bytes.clone());
    let request = Request::builder()
        .method(Method::POST)
        // No authorization header when auth is not required
        .uri("http://router.helicone.com/router/my-router/chat/completions")
        .body(request_body)
        .unwrap();
    let response = harness.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
