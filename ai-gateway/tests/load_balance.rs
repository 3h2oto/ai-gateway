use std::collections::HashMap;

use ai_gateway::{
    config::{
        Config,
        balance::{BalanceConfig, BalanceConfigInner},
        helicone::HeliconeFeatures,
        router::{RouterConfig, RouterConfigs},
    },
    endpoints::EndpointType,
    tests::{TestDefault, harness::Harness, mock::MockArgs},
    types::{provider::InferenceProvider, router::RouterId},
};
use compact_str::CompactString;
use http::{Method, Request, StatusCode};
use nonempty_collections::nes;
use serde_json::json;
use tower::Service;

fn p2c_config_openai_anthropic_google() -> RouterConfigs {
    RouterConfigs::new(HashMap::from([(
        RouterId::Named(CompactString::new("my-router")),
        RouterConfig {
            load_balance: BalanceConfig(HashMap::from([(
                EndpointType::Chat,
                BalanceConfigInner::BalancedLatency {
                    providers: nes![
                        InferenceProvider::OpenAI,
                        InferenceProvider::Anthropic,
                        InferenceProvider::GoogleGemini
                    ],
                },
            )])),
            model_mappings: None,
            cache: None,
            retries: None,
            rate_limit: None,
            providers: None,
        },
    )]))
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "issue with stubr latency not working correctly"]
async fn openai_slow() {
    let mut config = Config::test_default();
    // Disable auth for this test since we're testing load balancing behavior
    config.helicone.features = HeliconeFeatures::None;
    // Use p2c balance config with OpenAI, Anthropic, and Google providers
    config.routers = p2c_config_openai_anthropic_google();
    let latency = 100;
    let requests = 100;
    let mock_args = MockArgs::builder()
        .stubs(HashMap::from([
            ("success:openai:chat_completion", (..40).into()),
            ("success:anthropic:messages", (30..).into()),
            ("success:gemini:generate_content", (30..).into()),
            ("success:minio:upload_request", 0.into()),
            ("success:jawn:log_request", 0.into()),
        ]))
        .global_openai_latency(latency)
        .verify(false)
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

    for _ in 0..requests {
        let request_body = axum_core::body::Body::from(body_bytes.clone());
        let request = Request::builder()
            .method(Method::POST)
            // default router
            .uri("http://router.helicone.com/router/my-router/chat/completions")
            .body(request_body)
            .unwrap();
        let response = harness.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "issue with stubr latency not working correctly"]
async fn anthropic_slow() {
    let mut config = Config::test_default();
    // Disable auth for this test since we're testing load balancing behavior
    config.helicone.features = HeliconeFeatures::None;
    // Use p2c balance config with OpenAI, Anthropic, and Google providers
    config.routers = p2c_config_openai_anthropic_google();
    let latency = 10;
    let requests = 100;
    let mock_args = MockArgs::builder()
        .stubs(HashMap::from([
            ("success:openai:chat_completion", (30..).into()),
            ("success:anthropic:messages", (..60).into()),
            ("success:gemini:generate_content", (..60).into()),
            ("success:minio:upload_request", 0.into()),
            ("success:jawn:log_request", 0.into()),
        ]))
        .global_anthropic_latency(latency)
        .verify(false)
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

    for _ in 0..requests {
        let request_body = axum_core::body::Body::from(body_bytes.clone());
        let request = Request::builder()
            .method(Method::POST)
            // default router
            .uri("http://router.helicone.com/router/my-router/chat/completions")
            .body(request_body)
            .unwrap();
        let response = harness.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
