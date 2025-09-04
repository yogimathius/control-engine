use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde_json::json;
use std::time::Instant;
use uuid::Uuid;

use crate::{
    auth::{create_auth_response, hash_password, verify_password},
    models::*,
    reflection::{Reflector, ReflectionConfig},
    state::ArchetypalState,
};

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub engine: std::sync::Arc<crate::CodexEngine>,
}

pub async fn register_user(
    State(app_state): State<AppState>,
    Json(registration): Json<PractitionerRegistration>,
) -> Result<Json<SuccessResponse<AuthToken>>, (StatusCode, Json<ErrorResponse>)> {
    // Validate email doesn't already exist
    let existing = sqlx::query("SELECT id FROM practitioners WHERE email = $1")
        .bind(&registration.email)
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Sacred practitioner with this email already exists".to_string(),
            }),
        ));
    }

    // Hash password
    let password_hash = hash_password(&registration.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Password hashing failed: {}", e),
            }),
        )
    })?;

    // Create new practitioner
    let practitioner_id = Uuid::new_v4();
    let practitioner = sqlx::query_as::<_, Practitioner>(
        r#"
        INSERT INTO practitioners (id, email, password_hash, spiritual_name, sacred_path, 
                                 archetypal_preferences, energy_alignments, privacy_level)
        VALUES ($1, $2, $3, $4, $5, '{}', '{}', 'private')
        RETURNING *
        "#,
    )
    .bind(practitioner_id)
    .bind(&registration.email)
    .bind(&password_hash)
    .bind(&registration.spiritual_name)
    .bind(&registration.sacred_path)
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to create sacred practitioner: {}", e),
            }),
        )
    })?;

    // Create authentication token
    let auth_token = create_auth_response(&practitioner).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Token creation failed: {}", e),
            }),
        )
    })?;

    Ok(Json(SuccessResponse::new(auth_token)))
}

pub async fn login_user(
    State(app_state): State<AppState>,
    Json(login): Json<PractitionerLogin>,
) -> Result<Json<SuccessResponse<AuthToken>>, (StatusCode, Json<ErrorResponse>)> {
    // Find practitioner by email
    let practitioner =
        sqlx::query_as::<_, Practitioner>("SELECT * FROM practitioners WHERE email = $1")
            .bind(&login.email)
            .fetch_one(&app_state.db)
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Invalid sacred credentials".to_string(),
                    }),
                )
            })?;

    // Verify password
    let password_valid =
        verify_password(&login.password, &practitioner.password_hash).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Password verification failed: {}", e),
                }),
            )
        })?;

    if !password_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid sacred credentials".to_string(),
            }),
        ));
    }

    // Create authentication token
    let auth_token = create_auth_response(&practitioner).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Token creation failed: {}", e),
            }),
        )
    })?;

    Ok(Json(SuccessResponse::new(auth_token)))
}

pub async fn get_profile(
    Extension(practitioner): Extension<Practitioner>,
) -> Json<SuccessResponse<PractitionerProfile>> {
    let profile = PractitionerProfile {
        id: practitioner.id,
        email: practitioner.email,
        spiritual_name: practitioner.spiritual_name,
        archetypal_preferences: practitioner.archetypal_preferences,
        energy_alignments: practitioner.energy_alignments,
        privacy_level: practitioner.privacy_level,
        sacred_path: practitioner.sacred_path,
        member_since: practitioner.created_at,
    };

    Json(SuccessResponse::new(profile))
}

pub async fn execute_ritual(
    State(app_state): State<AppState>,
    Extension(practitioner): Extension<Practitioner>,
    Json(request): Json<RitualExecutionRequest>,
) -> Result<Json<SuccessResponse<TransformationResult>>, (StatusCode, Json<ErrorResponse>)> {
    let execution_start = Instant::now();

    // Get current practitioner state
    let current_state = get_practitioner_current_state(&app_state.db, practitioner.id).await?;

    // Execute ritual using the engine (simplified for now)
    let mut post_state = current_state.clone();

    // Apply basic ritual transformations based on ritual name
    match request.ritual_name.as_str() {
        "shadow_integration" => {
            let shadow_val = post_state.archetypes.get("Shadow").unwrap_or(&0.0);
            post_state
                .archetypes
                .insert("Shadow".to_string(), shadow_val + 0.2);
            post_state.symbols.push("◯●◯".to_string());
        }
        "archetype_invocation" => {
            for (_, activation) in post_state.archetypes.iter_mut() {
                *activation += 0.1;
            }
            post_state.symbols.push("🔮".to_string());
        }
        "energy_attunement" => {
            for (_, energy) in post_state.energies.iter_mut() {
                *energy = (*energy + 0.1).min(1.0);
            }
            post_state.symbols.push("∿∿∿".to_string());
        }
        "void_contemplation" => {
            let void_val = post_state.energies.get("Void").unwrap_or(&0.0);
            post_state
                .energies
                .insert("Void".to_string(), void_val + 0.3);
            post_state.symbols.push("○".to_string());
        }
        _ => {
            // Default transformation
            for (_, activation) in post_state.archetypes.iter_mut() {
                *activation += 0.05;
            }
        }
    }

    // Ritual execution is complete

    let execution_duration = execution_start.elapsed();

    // Calculate transformation intensity
    let transformation_intensity = calculate_transformation_intensity(&current_state, &post_state);

    // Store the new state
    let post_state_id = store_archetypal_state(&app_state.db, practitioner.id, &post_state).await?;

    // Create session record
    let session_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO ritual_sessions (id, practitioner_id, ritual_id, pre_state_id, post_state_id,
                                   execution_duration_ms, transformation_intensity)
        VALUES ($1, $2, 
                (SELECT id FROM sacred_rituals WHERE name = $3 LIMIT 1),
                (SELECT id FROM archetypal_states WHERE practitioner_id = $4 ORDER BY created_at DESC LIMIT 1 OFFSET 1),
                $5, $6, $7)
        "#,
    )
    .bind(session_id)
    .bind(practitioner.id)
    .bind(&request.ritual_name)
    .bind(practitioner.id)
    .bind(post_state_id)
    .bind(execution_duration.as_millis() as i32)
    .bind(transformation_intensity)
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to record ritual session: {}", e),
            }),
        )
    })?;

    // Generate suggestions and symbols
    let emerged_symbols = generate_emerged_symbols(&current_state, &post_state);
    let integration_required = generate_integration_suggestions(&post_state);
    let next_rituals_suggested = suggest_next_rituals(&post_state);

    let result = TransformationResult {
        session_id,
        pre_state: current_state,
        post_state,
        transformation_intensity,
        emerged_symbols,
        integration_required,
        next_rituals_suggested,
        oracle_consultation_recommended: transformation_intensity > 0.7,
        execution_duration_ms: execution_duration.as_millis(),
    };

    Ok(Json(SuccessResponse::new(result)))
}

pub async fn get_ritual_catalog(
    State(app_state): State<AppState>,
) -> Result<Json<SuccessResponse<Vec<SacredRitual>>>, (StatusCode, Json<ErrorResponse>)> {
    let rituals = sqlx::query_as::<_, SacredRitual>(
        "SELECT id, name, description, intent, tradition, difficulty_level, required_archetypes, 
         energy_requirements, wasm_module_data, wasm_module_hash, module_language, author_id,
         usage_count, effectiveness_rating::double precision as effectiveness_rating, 
         rating_count, is_public, tags, created_at, updated_at 
         FROM sacred_rituals WHERE is_public = true ORDER BY usage_count DESC, created_at DESC"
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch ritual catalog: {}", e),
            }),
        )
    })?;

    Ok(Json(SuccessResponse::new(rituals)))
}

pub async fn upload_ritual(
    State(app_state): State<AppState>,
    Extension(practitioner): Extension<Practitioner>,
    Json(upload): Json<RitualUpload>,
) -> Result<Json<SuccessResponse<SacredRitual>>, (StatusCode, Json<ErrorResponse>)> {
    let ritual_id = Uuid::new_v4();

    let ritual = sqlx::query_as::<_, SacredRitual>(
        r#"
        INSERT INTO sacred_rituals (id, name, description, intent, tradition, difficulty_level,
                                  required_archetypes, energy_requirements, wasm_module_data,
                                  module_language, author_id, is_public)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
    )
    .bind(ritual_id)
    .bind(&upload.name)
    .bind(&upload.description)
    .bind(&upload.intent)
    .bind(&upload.tradition)
    .bind(&upload.difficulty_level)
    .bind(serde_json::to_value(&upload.required_archetypes).unwrap())
    .bind(serde_json::to_value(&upload.energy_requirements).unwrap())
    .bind(upload.wasm_module.as_deref())
    .bind(upload.module_language.as_deref())
    .bind(practitioner.id)
    .bind(upload.is_public)
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to upload ritual: {}", e),
            }),
        )
    })?;

    Ok(Json(SuccessResponse::new(ritual)))
}

pub async fn get_ritual_details(
    State(app_state): State<AppState>,
    Path(ritual_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<SacredRitual>>, (StatusCode, Json<ErrorResponse>)> {
    let ritual = sqlx::query_as::<_, SacredRitual>("SELECT * FROM sacred_rituals WHERE id = $1")
        .bind(ritual_id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Sacred ritual not found".to_string(),
                }),
            )
        })?;

    Ok(Json(SuccessResponse::new(ritual)))
}

pub async fn get_current_state(
    State(app_state): State<AppState>,
    Extension(practitioner): Extension<Practitioner>,
) -> Result<Json<SuccessResponse<crate::state::ArchetypalState>>, (StatusCode, Json<ErrorResponse>)>
{
    let state = get_practitioner_current_state(&app_state.db, practitioner.id).await?;
    Ok(Json(SuccessResponse::new(state)))
}

pub async fn transform_state(
    State(app_state): State<AppState>,
    Extension(practitioner): Extension<Practitioner>,
    Json(request): Json<StateTransformationRequest>,
) -> Result<Json<SuccessResponse<crate::state::ArchetypalState>>, (StatusCode, Json<ErrorResponse>)>
{
    // Get current state
    let mut current_state = get_practitioner_current_state(&app_state.db, practitioner.id).await?;

    // Apply transformation based on type
    match request.transformation_type.as_str() {
        "archetype_activation" => {
            if let Some(archetype_name) = request.parameters.get("archetype") {
                if let Some(intensity) = request.parameters.get("intensity") {
                    let archetype = archetype_name.as_str().unwrap_or("");
                    let intensity_val = intensity.as_f64().unwrap_or(0.1);
                    current_state
                        .archetypes
                        .insert(archetype.to_string(), intensity_val);
                }
            }
        }
        "energy_adjustment" => {
            if let Some(energy_type) = request.parameters.get("energy_type") {
                if let Some(adjustment) = request.parameters.get("adjustment") {
                    let energy = energy_type.as_str().unwrap_or("");
                    let adjustment_val = adjustment.as_f64().unwrap_or(0.0);
                    let current_val = current_state.energies.get(energy).unwrap_or(&0.0);
                    current_state
                        .energies
                        .insert(energy.to_string(), current_val + adjustment_val);
                }
            }
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Unknown transformation type".to_string(),
                }),
            ));
        }
    }

    // Store the updated state
    store_archetypal_state(&app_state.db, practitioner.id, &current_state).await?;

    Ok(Json(SuccessResponse::new(current_state)))
}

pub async fn get_state_history(
    State(app_state): State<AppState>,
    Extension(practitioner): Extension<Practitioner>,
) -> Result<Json<SuccessResponse<Vec<StoredState>>>, (StatusCode, Json<ErrorResponse>)> {
    let states = sqlx::query_as::<_, StoredState>(
        "SELECT * FROM archetypal_states WHERE practitioner_id = $1 ORDER BY created_at DESC LIMIT 20"
    )
    .bind(practitioner.id)
    .fetch_all(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch state history: {}", e),
            }),
        )
    })?;

    Ok(Json(SuccessResponse::new(states)))
}

pub async fn request_reflection(
    State(app_state): State<AppState>,
    Extension(practitioner): Extension<Practitioner>,
    Json(request): Json<ReflectionRequest>,
) -> Result<Json<SuccessResponse<OracleInsight>>, (StatusCode, Json<ErrorResponse>)> {
    // Create AI reflector with configuration
    let reflection_config = ReflectionConfig::default();
    let reflector = Reflector::new(reflection_config);
    
    // If session_id is provided, fetch ritual session for context
    let ritual_context = if let Some(session_id) = request.session_id {
        // Get ritual session from database
        match sqlx::query_as::<_, RitualSessionRecord>(
            "SELECT * FROM ritual_sessions WHERE id = $1 AND practitioner_id = $2"
        )
        .bind(session_id)
        .bind(practitioner.id)
        .fetch_optional(&app_state.db)
        .await
        {
            Ok(Some(session)) => {
                // Get the ritual details
                let ritual = sqlx::query_as::<_, SacredRitual>(
                    "SELECT * FROM sacred_rituals WHERE id = $1"
                )
                .bind(session.ritual_id)
                .fetch_optional(&app_state.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: format!("Failed to fetch ritual: {}", e),
                        }),
                    )
                })?;
                
                if let Some(ritual) = ritual {
                    Some((session, ritual))
                } else {
                    None
                }
            },
            Ok(None) => None,
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to fetch ritual session: {}", e),
                    }),
                ));
            }
        }
    } else {
        None
    };
    
    // Get practitioner's current state
    let current_state = get_practitioner_current_state(&app_state.db, practitioner.id).await.ok();
    
    // Create mock ritual result for AI analysis (in future, this would come from actual ritual execution)
    let ritual_result = if let Some((session, ritual)) = ritual_context {
        crate::ritual::RitualResult {
            ritual_name: ritual.name.clone(),
            execution_id: session.id,
            timestamp: session.created_at,
            duration_ms: session.execution_duration_ms.unwrap_or(0) as u64,
            symbolic_outputs: std::collections::HashMap::new(),
            state_changes: vec![],
            emergent_symbols: vec!["🔮".to_string(), "∞".to_string(), "⚡".to_string()],
            completion_status: crate::ritual::CompletionStatus::Complete,
            resonance_level: session.transformation_intensity.unwrap_or(0.5),
        }
    } else {
        // Create a generic reflection request
        crate::ritual::RitualResult {
            ritual_name: "general_reflection".to_string(),
            execution_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
            symbolic_outputs: std::collections::HashMap::new(),
            state_changes: vec![],
            emergent_symbols: vec!["🔮".to_string(), "∞".to_string(), "⚡".to_string()],
            completion_status: crate::ritual::CompletionStatus::Complete,
            resonance_level: 0.7,
        }
    };
    
    // Create a SymbolicState for reflection analysis 
    // In the future, this would be converted from ArchetypalState or retrieved directly
    let symbolic_state = crate::state::SymbolicState::new();
    
    // Get AI reflection
    match reflector.reflect_on_ritual(&ritual_result, &symbolic_state).await {
        Ok(reflection) => {
            // Convert ReflectionResult to OracleInsight and store in database
            let insight_id = Uuid::new_v4();
            
            let oracle_insight = OracleInsight {
                id: insight_id,
                session_id: request.session_id,
                insight_type: "ai_reflection".to_string(),
                archetypal_analysis: json!({
                    "interpretation": reflection.archetypal_interpretation,
                    "symbolic_meaning": reflection.symbolic_meaning,
                    "resonance_level": ritual_result.resonance_level
                }),
                integration_suggestions: json!({
                    "guidance": reflection.integration_guidance,
                    "insights": reflection.emergent_insights,
                    "next_steps": reflection.next_steps
                }),
                symbolic_emergence: json!({
                    "symbols": ritual_result.emergent_symbols,
                    "resonance_analysis": reflection.resonance_analysis
                }),
                oracle_model: std::env::var("DEFAULT_AI_MODEL").unwrap_or("anthropic/claude-3-haiku".to_string()),
                confidence_score: 0.85,
                created_at: chrono::Utc::now(),
            };
            
            // Store insight in database
            sqlx::query(
                r#"INSERT INTO oracle_insights 
                   (id, session_id, insight_type, archetypal_analysis, integration_suggestions, 
                    symbolic_emergence, oracle_model, confidence_score, created_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#
            )
            .bind(oracle_insight.id)
            .bind(oracle_insight.session_id)
            .bind(&oracle_insight.insight_type)
            .bind(&oracle_insight.archetypal_analysis)
            .bind(&oracle_insight.integration_suggestions)
            .bind(&oracle_insight.symbolic_emergence)
            .bind(&oracle_insight.oracle_model)
            .bind(oracle_insight.confidence_score)
            .bind(oracle_insight.created_at)
            .execute(&app_state.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to store oracle insight: {}", e),
                    }),
                )
            })?;
            
            Ok(Json(SuccessResponse::new(oracle_insight)))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("AI reflection failed: {}", e),
                }),
            ))
        }
    }
}

// Helper functions

async fn get_practitioner_current_state(
    db: &sqlx::PgPool,
    practitioner_id: Uuid,
) -> Result<crate::state::ArchetypalState, (StatusCode, Json<ErrorResponse>)> {
    let stored_state = sqlx::query_as::<_, StoredState>(
        "SELECT * FROM archetypal_states WHERE practitioner_id = $1 ORDER BY created_at DESC LIMIT 1"
    )
    .bind(practitioner_id)
    .fetch_optional(db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch current state: {}", e),
            }),
        )
    })?;

    match stored_state {
        Some(state) => {
            // Convert stored state to ArchetypalState
            let archetypal_state = crate::state::ArchetypalState {
                archetypes: serde_json::from_value(state.archetypes).unwrap_or_default(),
                energies: serde_json::from_value(state.energies).unwrap_or_default(),
                integrations: serde_json::from_value(state.integrations).unwrap_or_default(),
                symbols: serde_json::from_value(state.symbols).unwrap_or_default(),
                transformations: serde_json::from_value(state.transformations).unwrap_or_default(),
            };
            Ok(archetypal_state)
        }
        None => {
            // Create initial state
            let initial_state = ArchetypalState::new();
            store_archetypal_state(db, practitioner_id, &initial_state).await?;
            Ok(initial_state)
        }
    }
}

async fn store_archetypal_state(
    db: &sqlx::PgPool,
    practitioner_id: Uuid,
    state: &ArchetypalState,
) -> Result<Uuid, (StatusCode, Json<ErrorResponse>)> {
    let state_id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO archetypal_states (id, practitioner_id, state_data, archetypes, energies, 
                                     integrations, symbols, transformations)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(state_id)
    .bind(practitioner_id)
    .bind(serde_json::to_value(state).unwrap())
    .bind(serde_json::to_value(&state.archetypes).unwrap())
    .bind(serde_json::to_value(&state.energies).unwrap())
    .bind(serde_json::to_value(&state.integrations).unwrap())
    .bind(serde_json::to_value(&state.symbols).unwrap())
    .bind(serde_json::to_value(&state.transformations).unwrap())
    .execute(db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to store state: {}", e),
            }),
        )
    })?;

    Ok(state_id)
}

fn calculate_transformation_intensity(
    pre_state: &ArchetypalState,
    post_state: &ArchetypalState,
) -> f64 {
    let mut total_change = 0.0;
    let mut change_count = 0;

    // Calculate archetype changes
    for (archetype, &post_value) in &post_state.archetypes {
        let pre_value = pre_state.archetypes.get(archetype).unwrap_or(&0.0);
        total_change += (post_value - pre_value).abs();
        change_count += 1;
    }

    // Calculate energy changes
    for (energy, &post_value) in &post_state.energies {
        let pre_value = pre_state.energies.get(energy).unwrap_or(&0.0);
        total_change += (post_value - pre_value).abs();
        change_count += 1;
    }

    if change_count > 0 {
        total_change / change_count as f64
    } else {
        0.0
    }
}

fn generate_emerged_symbols(
    _pre_state: &ArchetypalState,
    post_state: &ArchetypalState,
) -> Vec<String> {
    // Generate symbols based on dominant energies/archetypes
    let mut symbols = Vec::new();

    for (archetype, &strength) in &post_state.archetypes {
        if strength > 0.5 {
            match archetype.as_str() {
                "Sage" => symbols.push("🔮".to_string()),
                "Creator" => symbols.push("∆∇∆".to_string()),
                "Shadow" => symbols.push("◯●◯".to_string()),
                _ => {}
            }
        }
    }

    symbols
}

fn generate_integration_suggestions(_state: &ArchetypalState) -> Vec<String> {
    vec![
        "Daily meditation practice".to_string(),
        "Shadow journaling exercises".to_string(),
        "Creative expression work".to_string(),
    ]
}

fn suggest_next_rituals(state: &crate::state::ArchetypalState) -> Vec<String> {
    let mut suggestions = Vec::new();

    let total_energy: f64 = state.energies.values().sum();
    if total_energy > 3.0 {
        suggestions.push("energy_integration".to_string());
    }

    let shadow_strength = state.archetypes.get("Shadow").unwrap_or(&0.0);
    let light_strength = state.archetypes.get("Light").unwrap_or(&0.0);
    if (shadow_strength - light_strength).abs() > 0.3 {
        suggestions.push("shadow_integration".to_string());
    }

    suggestions
}
