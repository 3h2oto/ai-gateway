use chrono::{DateTime, Utc};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::router::RouterConfig;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RouterId {
    #[serde(untagged)]
    Named(CompactString),
}

impl AsRef<str> for RouterId {
    fn as_ref(&self) -> &str {
        match self {
            RouterId::Named(name) => name.as_str(),
        }
    }
}

impl std::fmt::Display for RouterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterId::Named(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug)]
pub struct VersionedRouter {
    pub version_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub version: String,
    pub config: RouterConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_id_round_trip() {
        let id = RouterId::Named(CompactString::new("test_name"));
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized =
            serde_json::from_str::<RouterId>(&serialized).unwrap();
        assert_eq!(id, deserialized);

        let id = RouterId::Named(CompactString::new("my-router"));
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized =
            serde_json::from_str::<RouterId>(&serialized).unwrap();
        assert_eq!(id, deserialized);
    }
}
