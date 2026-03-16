//! Runtime registry for MOD service RPC endpoints.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Tracks registered MOD service RPC endpoints at runtime.
///
/// Separate from `ModInfo` / `ModManifest` (statically resolved from `package.json`).
/// RPC registrations are dynamic — they appear when a MOD service starts.
#[derive(Default, Debug, Clone)]
pub struct RpcRegistry {
    entries: HashMap<String, RpcRegistration>,
}

/// A single MOD's RPC registration.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcRegistration {
    pub port: u16,
    pub methods: HashMap<String, RpcMethodMeta>,
}

/// Metadata for a single RPC method.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMethodMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

/// Shared reference to the RPC registry, usable across async boundaries.
///
/// Wraps `Arc<RwLock<RpcRegistry>>` as a Bevy `Resource` so it can be
/// shared between the Bevy world (MOD spawning) and Axum handlers (HTTP proxy).
#[derive(Resource, Clone)]
pub struct SharedRpcRegistry(pub Arc<RwLock<RpcRegistry>>);

impl Default for SharedRpcRegistry {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(RpcRegistry::default())))
    }
}

impl std::ops::Deref for SharedRpcRegistry {
    type Target = Arc<RwLock<RpcRegistry>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RpcRegistry {
    /// Register (or re-register) a MOD service's RPC endpoint.
    pub fn register(
        &mut self,
        mod_name: String,
        port: u16,
        methods: HashMap<String, RpcMethodMeta>,
    ) {
        self.entries
            .insert(mod_name, RpcRegistration { port, methods });
    }

    /// Remove a MOD service's registration.
    pub fn deregister(&mut self, mod_name: &str) {
        self.entries.remove(mod_name);
    }

    /// Look up a MOD's RPC registration.
    pub fn get(&self, mod_name: &str) -> Option<&RpcRegistration> {
        self.entries.get(mod_name)
    }

    /// Returns all registered entries.
    pub fn all(&self) -> &HashMap<String, RpcRegistration> {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_lookup() {
        let mut reg = RpcRegistry::default();
        let mut methods = HashMap::new();
        methods.insert(
            "show".to_string(),
            RpcMethodMeta {
                description: Some("Display visual".to_string()),
                timeout: Some(15000),
            },
        );
        reg.register("visual".to_string(), 54321, methods);

        let entry = reg.get("visual").unwrap();
        assert_eq!(entry.port, 54321);
        assert_eq!(entry.methods.len(), 1);
        assert_eq!(entry.methods["show"].timeout, Some(15000));
    }

    #[test]
    fn lookup_missing_returns_none() {
        let reg = RpcRegistry::default();
        assert!(reg.get("nonexistent").is_none());
    }

    #[test]
    fn deregister_removes_entry() {
        let mut reg = RpcRegistry::default();
        reg.register("visual".to_string(), 54321, HashMap::new());
        assert!(reg.get("visual").is_some());
        reg.deregister("visual");
        assert!(reg.get("visual").is_none());
    }

    #[test]
    fn re_register_overwrites() {
        let mut reg = RpcRegistry::default();
        let mut m1 = HashMap::new();
        m1.insert("a".to_string(), RpcMethodMeta::default());
        reg.register("mod1".to_string(), 1000, m1);

        let mut m2 = HashMap::new();
        m2.insert("b".to_string(), RpcMethodMeta::default());
        reg.register("mod1".to_string(), 2000, m2);

        let entry = reg.get("mod1").unwrap();
        assert_eq!(entry.port, 2000);
        assert!(entry.methods.contains_key("b"));
        assert!(!entry.methods.contains_key("a"));
    }

    #[test]
    fn all_returns_all_entries() {
        let mut reg = RpcRegistry::default();
        reg.register("a".to_string(), 1000, HashMap::new());
        reg.register("b".to_string(), 2000, HashMap::new());
        assert_eq!(reg.all().len(), 2);
    }
}
