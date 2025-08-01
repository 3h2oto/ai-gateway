use displaydoc::Display;
use telemetry::TelemetryError;
use thiserror::Error;

use crate::{
    config::validation::ModelMappingValidationError,
    types::{provider::InferenceProvider, router::RouterId},
};

/// Errors that can occur during initialization.
#[derive(Debug, Error, Display)]
pub enum InitError {
    /// Default router not found
    DefaultRouterNotFound,
    /// Failed to read TLS certificate: {0}
    Tls(std::io::Error),
    /// Failed to bind to address: {0}
    Bind(std::io::Error),
    /// Telemetry: {0}
    Telemetry(#[from] TelemetryError),
    /// Invalid bucket config: {0}
    InvalidBucketConfig(#[from] rusty_s3::BucketError),
    /// OAuth config: {0}
    OAuthConfig(url::ParseError),
    /// Failed to create reqwest client: {0}
    CreateReqwestClient(reqwest::Error),
    /// Failed to create balancer: {0}
    CreateBalancer(tower::BoxError),
    /// Provider error: {0}
    ProviderError(#[from] crate::error::provider::ProviderError),
    /// Invalid weight for provider: {0}
    InvalidWeight(InferenceProvider),
    /// Invalid balancer: {0}
    InvalidBalancer(String),
    /// Converter registry endpoints not configured for provider: {0}
    EndpointsNotConfigured(InferenceProvider),
    /// Failed to create redis pool: {0}
    CreateRedisPool(#[from] r2d2::Error),
    /// Failed to create redis client: {0}
    CreateRedisClient(#[from] redis::RedisError),
    /// Failed to build otel metrics layer: {0}
    InitOtelMetricsLayer(#[from] tower_otel_http_metrics::Error),
    /// Failed to initialize system metrics
    InitSystemMetrics,
    /// Invalid rate limit config: {0}
    InvalidRateLimitConfig(&'static str),
    /// Invalid mappings config: {0}
    InvalidMappingsConfig(#[from] ModelMappingValidationError),
    /// Failed to connect to websocket: {0}
    WebsocketConnection(#[from] Box<tokio_tungstenite::tungstenite::Error>),
    /// URL parsing error: {0}
    WebsocketUrlParse(#[from] url::ParseError),
    /// Rate limit channels not initialized for router: {0}
    RateLimitChannelsNotInitialized(RouterId),
    /// Failed to build websocket request: {0}
    WebsocketRequestBuild(#[from] http::Error),
    /// Invalid router id: {0}
    InvalidRouterId(String),
    /// Cache not configured
    CacheNotConfigured,
    /// Minio not configured
    MinioNotConfigured,
    /// Database connection error: {0}
    DatabaseConnection(sqlx::Error),
    /// Model ID not recognized: {0}
    ModelIdNotRecognized(String),
    /// Provider not yet supported: {0}
    ProviderNotSupported(InferenceProvider),
    /// Router rx not configured
    RouterRxNotConfigured,
    /// Store not configured: {0}
    StoreNotConfigured(&'static str),
    /// Router api keys not initialized
    RouterApiKeysNotInitialized,
    /// Invalid organization id: {0}
    InvalidOrganizationId(String),
    /// Router tx not set
    RouterTxNotSet,
    /// Database listener only compatible with cloud deployment target
    DatabaseListenerOnlyCloud,
    /// Failed to load initial helicone api keys from db: {0}
    InitHeliconeKeys(String),
}
