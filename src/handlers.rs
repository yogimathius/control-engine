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
        "SELECT * FROM sacred_rituals WHERE is_public = true ORDER BY usage_count DESC, created_at DESC"
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
    State(_app_state): State<AppState>,
    Extension(_practitioner): Extension<Practitioner>,
    Json(request): Json<ReflectionRequest>,
) -> Result<Json<SuccessResponse<OracleInsight>>, (StatusCode, Json<ErrorResponse>)> {
    // For now, return a mock insight
    // TODO: Implement real AI integration
    let insight_id = Uuid::new_v4();
    let insight = OracleInsight {
        id: insight_id,
        session_id: request.session_id,
        insight_type: "reflection".to_string(),
        archetypal_analysis: json!({
            "summary": "Sacred transformation detected",
            "dominant_archetypes": ["Sage", "Creator"],
            "integration_level": 0.75
        }),
        integration_suggestions: json!({
            "practices": ["morning meditation", "shadow journaling"],
            "next_phase": "deepening work"
        }),
        symbolic_emergence: json!({
            "symbols": ["🔮", "∞", "⚡"],
            "meanings": ["divine connection", "infinite potential", "energetic activation"]
        }),
        oracle_model: "mock_oracle".to_string(),
        confidence_score: 0.8,
        created_at: chrono::Utc::now(),
    };

    Ok(Json(SuccessResponse::new(insight)))
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
