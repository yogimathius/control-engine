use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Practitioner {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub spiritual_name: Option<String>,
    pub archetypal_preferences: serde_json::Value,
    pub energy_alignments: serde_json::Value,
    pub privacy_level: String,
    pub sacred_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerRegistration {
    pub email: String,
    pub password: String,
    pub spiritual_name: Option<String>,
    pub sacred_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SacredRitual {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub intent: String,
    pub tradition: String,
    pub difficulty_level: String,
    pub required_archetypes: serde_json::Value,
    pub energy_requirements: serde_json::Value,
    pub wasm_module_data: Option<Vec<u8>>,
    pub wasm_module_hash: Option<String>,
    pub module_language: Option<String>,
    pub author_id: Option<Uuid>,
    pub usage_count: i32,
    pub effectiveness_rating: f64,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualUpload {
    pub name: String,
    pub description: String,
    pub intent: String,
    pub tradition: String,
    pub difficulty_level: String,
    pub required_archetypes: Vec<String>,
    pub energy_requirements: HashMap<String, f64>,
    pub wasm_module: Option<Vec<u8>>,
    pub module_language: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RitualSessionRecord {
    pub id: Uuid,
    pub practitioner_id: Uuid,
    pub ritual_id: Uuid,
    pub pre_state_id: Option<Uuid>,
    pub post_state_id: Option<Uuid>,
    pub execution_duration_ms: Option<i32>,
    pub transformation_intensity: Option<f64>,
    pub subjective_experience: Option<String>,
    pub ai_interpretation: Option<String>,
    pub integration_notes: Option<String>,
    pub effectiveness_rating: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualExecutionRequest {
    pub ritual_name: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub intention: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationResult {
    pub session_id: Uuid,
    pub pre_state: crate::state::ArchetypalState,
    pub post_state: crate::state::ArchetypalState,
    pub transformation_intensity: f64,
    pub emerged_symbols: Vec<String>,
    pub integration_required: Vec<String>,
    pub next_rituals_suggested: Vec<String>,
    pub oracle_consultation_recommended: bool,
    pub execution_duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredState {
    pub id: Uuid,
    pub practitioner_id: Uuid,
    pub state_data: serde_json::Value,
    pub archetypes: serde_json::Value,
    pub energies: serde_json::Value,
    pub integrations: serde_json::Value,
    pub symbols: serde_json::Value,
    pub transformations: serde_json::Value,
    pub state_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransformationRequest {
    pub transformation_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionRequest {
    pub session_id: Option<Uuid>,
    pub custom_query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OracleInsight {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub insight_type: String,
    pub archetypal_analysis: serde_json::Value,
    pub integration_suggestions: serde_json::Value,
    pub symbolic_emergence: serde_json::Value,
    pub oracle_model: String,
    pub confidence_score: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub practitioner: PractitionerProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerProfile {
    pub id: Uuid,
    pub email: String,
    pub spiritual_name: Option<String>,
    pub archetypal_preferences: serde_json::Value,
    pub energy_alignments: serde_json::Value,
    pub privacy_level: String,
    pub sacred_path: Option<String>,
    pub member_since: DateTime<Utc>,
}
