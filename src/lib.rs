pub mod cli;
pub mod engine;
pub mod reflection;
pub mod ritual;
pub mod state;

// Web server modules
pub mod auth;
pub mod database;
pub mod handlers;
pub mod models;

pub use engine::CodexEngine;
pub use reflection::{ReflectionResult, Reflector};
pub use ritual::{Ritual, RitualDefinition, RitualResult};
pub use state::{Archetype, Element, Energy, Integration, SymbolicState};

// Core error types for the Codex system
#[derive(thiserror::Error, Debug)]
pub enum CodexError {
    #[error("Ritual not found: {name}")]
    RitualNotFound { name: String },

    #[error("State corruption detected: {reason}")]
    StateCorruption { reason: String },

    #[error("WASM execution failed: {error}")]
    WasmExecution { error: String },

    #[error("Reflection failed: {error}")]
    ReflectionFailed { error: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("WASM error: {0}")]
    Wasm(#[from] wasmtime::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}
