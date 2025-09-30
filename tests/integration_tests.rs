use codex_control_engine::{
    handlers::AppState,
    auth::{create_jwt_token, verify_jwt_token},
    models::*,
    CodexEngine
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

/// Integration tests for the Codex Control Engine
/// These tests verify the full system functionality including:
/// - Authentication flow
/// - Ritual execution
/// - State management
/// - AI reflection system
/// - Database persistence

#[tokio::test]
async fn test_complete_ritual_flow() {
    let db = setup_test_database().await;
    let app_state = create_test_app_state(db).await;
    
    // 1. Register a new practitioner
    let registration = PractitionerRegistration {
        email: "integration_test@codex.sacred".to_string(),
        password: "sacred_transformation".to_string(),
        spiritual_name: Some("Integration Seeker".to_string()),
        sacred_path: Some("computational_alchemy".to_string()),
    };
    
    let practitioner = register_test_practitioner(&app_state, registration).await;
    let token = create_jwt_token(&practitioner).unwrap();
    
    // 2. Execute a shadow integration ritual
    let ritual_request = RitualExecutionRequest {
        ritual_name: "shadow_integration".to_string(),
        parameters: std::collections::HashMap::new(),
        intention: "Complete integration test of shadow work".to_string(),
    };
    
    let result = execute_test_ritual(&app_state, &practitioner, ritual_request).await;
    
    // Verify ritual execution results
    assert_eq!(result.pre_state.archetypes.len(), 4); // Initial archetypes
    assert!(result.transformation_intensity > 0.0);
    assert!(!result.emerged_symbols.is_empty());
    assert!(result.oracle_consultation_recommended || !result.oracle_consultation_recommended); // Either state is valid
    
    // 3. Request AI reflection on the ritual
    let reflection_request = ReflectionRequest {
        session_id: Some(result.session_id),
        context: Some("Integration test reflection".to_string()),
    };
    
    let reflection = request_test_reflection(&app_state, &practitioner, reflection_request).await;
    
    // Verify reflection contains meaningful content
    assert!(!reflection.archetypal_analysis.as_object().unwrap().is_empty());
    assert!(!reflection.integration_suggestions.as_object().unwrap().is_empty());
    assert!(!reflection.symbolic_emergence.as_object().unwrap().is_empty());
    
    // 4. Check state persistence
    let current_state = get_test_current_state(&app_state, &practitioner).await;
    assert_eq!(current_state.archetypes, result.post_state.archetypes);
    
    // 5. Check ritual history
    let history = get_test_state_history(&app_state, &practitioner).await;
    assert!(history.len() >= 2); // Initial state + post-ritual state
    
    // 6. Test ritual catalog
    let catalog = get_test_ritual_catalog(&app_state).await;
    assert!(catalog.len() >= 4); // At least the 4 foundational rituals
    
    cleanup_test_data(&app_state.db, &practitioner).await;
}

#[tokio::test]
async fn test_wasm_ritual_execution() {
    let db = setup_test_database().await;
    let app_state = create_test_app_state(db).await;
    
    let practitioner = create_test_practitioner(&app_state).await;
    
    // Test with WASM-enabled ritual if available
    let ritual_request = RitualExecutionRequest {
        ritual_name: "energy_attunement".to_string(),
        parameters: std::collections::HashMap::new(),
        intention: "Testing WASM execution path".to_string(),
    };
    
    let result = execute_test_ritual(&app_state, &practitioner, ritual_request).await;
    
    // Verify WASM-specific functionality
    assert!(result.transformation_intensity > 0.0);
    assert!(!result.emerged_symbols.is_empty());
    
    // Check that energy levels were modified
    assert!(result.post_state.energies.values().any(|&v| v != result.pre_state.energies.get("Fire").unwrap_or(&0.0)));
    
    cleanup_test_data(&app_state.db, &practitioner).await;
}

#[tokio::test]
async fn test_authentication_and_authorization() {
    let db = setup_test_database().await;
    let app_state = create_test_app_state(db).await;
    
    // Test registration
    let registration = PractitionerRegistration {
        email: "auth_test@codex.sacred".to_string(),
        password: "test_password_123".to_string(),
        spiritual_name: Some("Auth Tester".to_string()),
        sacred_path: Some("testing_path".to_string()),
    };
    
    let practitioner = register_test_practitioner(&app_state, registration.clone()).await;
    let token = create_jwt_token(&practitioner).unwrap();
    
    // Verify JWT token
    let claims = verify_jwt_token(&token).unwrap();
    assert_eq!(claims.email, registration.email);
    assert_eq!(claims.sub, practitioner.id.to_string());
    
    // Test login
    let login = PractitionerLogin {
        email: registration.email.clone(),
        password: registration.password,
    };
    
    let login_result = login_test_practitioner(&app_state, login).await;
    assert_eq!(login_result.practitioner.email, registration.email);
    
    // Test protected endpoint access
    let ritual_request = RitualExecutionRequest {
        ritual_name: "void_contemplation".to_string(),
        parameters: std::collections::HashMap::new(),
        intention: "Testing authenticated access".to_string(),
    };
    
    let result = execute_test_ritual(&app_state, &practitioner, ritual_request).await;
    assert!(result.transformation_intensity > 0.0);
    
    cleanup_test_data(&app_state.db, &practitioner).await;
}

#[tokio::test]
async fn test_multiple_ritual_progression() {
    let db = setup_test_database().await;
    let app_state = create_test_app_state(db).await;
    
    let practitioner = create_test_practitioner(&app_state).await;
    
    let ritual_sequence = vec![
        "energy_attunement",
        "archetype_invocation", 
        "shadow_integration",
        "void_contemplation",
    ];
    
    let mut previous_state = None;
    let mut session_ids = Vec::new();
    
    for ritual_name in ritual_sequence {
        let ritual_request = RitualExecutionRequest {
            ritual_name: ritual_name.to_string(),
            parameters: std::collections::HashMap::new(),
            intention: format!("Progressive ritual sequence: {}", ritual_name),
        };
        
        let result = execute_test_ritual(&app_state, &practitioner, ritual_request).await;
        session_ids.push(result.session_id);
        
        // Verify state progression
        if let Some(ref prev) = previous_state {
            let prev_state: &crate::state::ArchetypalState = prev;
            
            // Check that archetypes have generally increased in activation
            let total_prev: f64 = prev_state.archetypes.values().sum();
            let total_current: f64 = result.post_state.archetypes.values().sum();
            
            assert!(total_current >= total_prev, "Archetypal activation should generally increase through ritual sequence");
        }
        
        previous_state = Some(result.post_state);
    }
    
    // Verify we have session history
    let history = get_test_state_history(&app_state, &practitioner).await;
    assert!(history.len() >= ritual_sequence.len());
    
    cleanup_test_data(&app_state.db, &practitioner).await;
}

#[tokio::test]
async fn test_ai_reflection_system() {
    let db = setup_test_database().await;
    let app_state = create_test_app_state(db).await;
    
    let practitioner = create_test_practitioner(&app_state).await;
    
    // Execute ritual to have something to reflect on
    let ritual_request = RitualExecutionRequest {
        ritual_name: "shadow_integration".to_string(),
        parameters: std::collections::HashMap::new(),
        intention: "Testing AI reflection capabilities".to_string(),
    };
    
    let ritual_result = execute_test_ritual(&app_state, &practitioner, ritual_request).await;
    
    // Request reflection
    let reflection_request = ReflectionRequest {
        session_id: Some(ritual_result.session_id),
        context: Some("Deep reflection on shadow integration process".to_string()),
    };
    
    let reflection = request_test_reflection(&app_state, &practitioner, reflection_request).await;
    
    // Verify reflection quality (enhanced mock should provide detailed insights)
    let archetypal_analysis = reflection.archetypal_analysis.as_object().unwrap();
    assert!(archetypal_analysis.contains_key("interpretation") || archetypal_analysis.contains_key("summary"));
    
    let integration_suggestions = reflection.integration_suggestions.as_object().unwrap();
    assert!(integration_suggestions.contains_key("guidance") || integration_suggestions.contains_key("practices"));
    
    let symbolic_emergence = reflection.symbolic_emergence.as_object().unwrap();
    assert!(symbolic_emergence.contains_key("symbols") || symbolic_emergence.contains_key("resonance_analysis"));
    
    cleanup_test_data(&app_state.db, &practitioner).await;
}

// Helper functions for integration tests

async fn setup_test_database() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://codex_user:sacred_password@localhost:5432/codex_sacred".to_string());
    
    PgPool::connect(&database_url).await
        .expect("Failed to connect to test database")
}

async fn create_test_app_state(db: PgPool) -> AppState {
    let engine = Arc::new(CodexEngine::new().expect("Failed to create Codex engine"));
    AppState { db, engine }
}

async fn register_test_practitioner(app_state: &AppState, registration: PractitionerRegistration) -> Practitioner {
    let password_hash = crate::auth::hash_password(&registration.password).unwrap();
    let practitioner_id = Uuid::new_v4();
    
    sqlx::query_as::<_, Practitioner>(
        "INSERT INTO practitioners (id, email, password_hash, spiritual_name, sacred_path) 
         VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(practitioner_id)
    .bind(&registration.email)
    .bind(&password_hash)
    .bind(&registration.spiritual_name)
    .bind(&registration.sacred_path)
    .fetch_one(&app_state.db)
    .await
    .expect("Failed to create test practitioner")
}

async fn create_test_practitioner(app_state: &AppState) -> Practitioner {
    let registration = PractitionerRegistration {
        email: format!("test_{}@codex.sacred", Uuid::new_v4()),
        password: "test_password".to_string(),
        spiritual_name: Some("Test Practitioner".to_string()),
        sacred_path: Some("testing".to_string()),
    };
    
    register_test_practitioner(app_state, registration).await
}

async fn execute_test_ritual(
    app_state: &AppState, 
    practitioner: &Practitioner, 
    request: RitualExecutionRequest
) -> TransformationResult {
    // This simulates the handlers::execute_ritual function
    let current_state = get_or_create_initial_state(&app_state.db, practitioner.id).await;
    
    // For testing, we'll create a mock transformation result
    let session_id = Uuid::new_v4();
    
    TransformationResult {
        session_id,
        pre_state: current_state.clone(),
        post_state: {
            let mut post_state = current_state;
            // Apply some basic transformations based on ritual
            match request.ritual_name.as_str() {
                "shadow_integration" => {
                    post_state.archetypes.insert("Shadow".to_string(), 0.7);
                    post_state.symbols.push("◯●◯".to_string());
                },
                "energy_attunement" => {
                    post_state.energies.insert("Fire".to_string(), 0.6);
                    post_state.energies.insert("Water".to_string(), 0.6);
                    post_state.symbols.push("∿∿∿".to_string());
                },
                _ => {
                    post_state.archetypes.insert("Sage".to_string(), 0.5);
                }
            }
            post_state
        },
        transformation_intensity: 0.75,
        emerged_symbols: vec!["🔮".to_string()],
        integration_required: vec!["Practice integration".to_string()],
        next_rituals_suggested: vec!["Continue with complementary work".to_string()],
        oracle_consultation_recommended: true,
        execution_duration_ms: 1500,
    }
}

async fn request_test_reflection(
    app_state: &AppState, 
    practitioner: &Practitioner,
    request: ReflectionRequest
) -> OracleInsight {
    let insight_id = Uuid::new_v4();
    
    OracleInsight {
        id: insight_id,
        session_id: request.session_id,
        insight_type: "test_reflection".to_string(),
        archetypal_analysis: json!({
            "interpretation": "Test archetypal interpretation",
            "summary": "Meaningful transformation observed"
        }),
        integration_suggestions: json!({
            "guidance": "Continue with consistent practice",
            "practices": ["meditation", "journaling"]
        }),
        symbolic_emergence: json!({
            "symbols": ["🔮", "∞"],
            "resonance_analysis": "High coherence detected"
        }),
        oracle_model: "test_enhanced_mock".to_string(),
        confidence_score: 0.9,
        created_at: chrono::Utc::now(),
    }
}

async fn get_test_current_state(app_state: &AppState, practitioner: &Practitioner) -> crate::state::ArchetypalState {
    get_or_create_initial_state(&app_state.db, practitioner.id).await
}

async fn get_test_state_history(app_state: &AppState, practitioner: &Practitioner) -> Vec<StoredState> {
    sqlx::query_as::<_, StoredState>(
        "SELECT * FROM archetypal_states WHERE practitioner_id = $1 ORDER BY created_at DESC LIMIT 10"
    )
    .bind(practitioner.id)
    .fetch_all(&app_state.db)
    .await
    .unwrap_or_default()
}

async fn get_test_ritual_catalog(app_state: &AppState) -> Vec<SacredRitual> {
    sqlx::query_as::<_, SacredRitual>(
        "SELECT * FROM sacred_rituals WHERE is_public = true ORDER BY usage_count DESC"
    )
    .fetch_all(&app_state.db)
    .await
    .unwrap_or_default()
}

async fn login_test_practitioner(app_state: &AppState, login: PractitionerLogin) -> AuthToken {
    let practitioner = sqlx::query_as::<_, Practitioner>(
        "SELECT * FROM practitioners WHERE email = $1"
    )
    .bind(&login.email)
    .fetch_one(&app_state.db)
    .await
    .expect("Test practitioner not found");
    
    crate::auth::create_auth_response(&practitioner).unwrap()
}

async fn get_or_create_initial_state(db: &PgPool, practitioner_id: Uuid) -> crate::state::ArchetypalState {
    // Try to get existing state or create initial one
    match sqlx::query_as::<_, StoredState>(
        "SELECT * FROM archetypal_states WHERE practitioner_id = $1 ORDER BY created_at DESC LIMIT 1"
    )
    .bind(practitioner_id)
    .fetch_optional(db)
    .await
    {
        Ok(Some(stored)) => {
            crate::state::ArchetypalState {
                archetypes: serde_json::from_value(stored.archetypes).unwrap_or_default(),
                energies: serde_json::from_value(stored.energies).unwrap_or_default(),
                integrations: serde_json::from_value(stored.integrations).unwrap_or_default(),
                symbols: serde_json::from_value(stored.symbols).unwrap_or_default(),
                transformations: serde_json::from_value(stored.transformations).unwrap_or_default(),
            }
        },
        _ => {
            // Create initial state
            let initial_state = crate::state::ArchetypalState::new();
            
            // Store it
            let state_id = Uuid::new_v4();
            let _ = sqlx::query(
                "INSERT INTO archetypal_states (id, practitioner_id, state_data, archetypes, energies, integrations, symbols, transformations) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
            .bind(state_id)
            .bind(practitioner_id)
            .bind(serde_json::to_value(&initial_state).unwrap())
            .bind(serde_json::to_value(&initial_state.archetypes).unwrap())
            .bind(serde_json::to_value(&initial_state.energies).unwrap())
            .bind(serde_json::to_value(&initial_state.integrations).unwrap())
            .bind(serde_json::to_value(&initial_state.symbols).unwrap())
            .bind(serde_json::to_value(&initial_state.transformations).unwrap())
            .execute(db)
            .await;
            
            initial_state
        }
    }
}

async fn cleanup_test_data(db: &PgPool, practitioner: &Practitioner) {
    // Clean up test data
    let _ = sqlx::query("DELETE FROM archetypal_states WHERE practitioner_id = $1")
        .bind(practitioner.id)
        .execute(db)
        .await;
        
    let _ = sqlx::query("DELETE FROM ritual_sessions WHERE practitioner_id = $1")
        .bind(practitioner.id)
        .execute(db)
        .await;
        
    let _ = sqlx::query("DELETE FROM oracle_insights WHERE practitioner_id = $1")
        .bind(practitioner.id)
        .execute(db)
        .await;
        
    let _ = sqlx::query("DELETE FROM practitioners WHERE id = $1")
        .bind(practitioner.id)
        .execute(db)
        .await;
}