# Control Engine - Backend Requirements

## **Current Status: Rust CLI Complete, Backend Expansion Needed**
- Rust CLI with symbolic state management and ritual execution ✅
- Local WASM runtime for extensible rituals ✅
- **Missing**: Web interface, ritual sharing, collaborative sacred space

---

## **Backend Technology: Rust + Axum + WebAssembly**

**Why Rust + Axum + WASM:**
- **Sacred performance** - Ultra-fast ritual execution with zero-cost abstractions
- **Memory safety** for handling archetypal state transformations
- **WASM runtime** for portable ritual modules across languages
- **Concurrent ritual processing** for multiple users simultaneously
- **Type safety** ensures symbolic state integrity
- **Cross-platform** WASM rituals run everywhere

---

## **Required API Endpoints**

```rust
// User sacred space management
POST /api/users/register        # Create sacred account
POST /api/users/login          # Enter sacred space
GET /api/users/profile         # View practitioner profile
PUT /api/users/archetypal_state # Update personal archetype configuration

// Ritual execution and sharing
POST /api/rituals/execute      # Execute ritual with state transformation
GET /api/rituals/catalog       # Browse available rituals
POST /api/rituals/upload       # Share custom WASM ritual
GET /api/rituals/:id/details   # Get ritual documentation
PUT /api/rituals/:id/rate      # Rate ritual effectiveness

// Symbolic state management
GET /api/state/current         # Get current symbolic state
POST /api/state/transform      # Apply state transformation
GET /api/state/history         # View transformation history
POST /api/state/reflection     # Get AI archetypal interpretation

// Collective sacred space (Phase 4)
POST /api/collective/rituals   # Participate in group rituals
GET /api/collective/spaces     # Available shared spaces
WebSocket /ws/collective       # Real-time collective consciousness
```

---

## **Database Schema (PostgreSQL + JSONB)**

```sql
-- Practitioners and their sacred profiles
CREATE TABLE practitioners (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    spiritual_name VARCHAR(100), -- chosen sacred name
    archetypal_preferences JSONB DEFAULT '{}', -- preferred archetypes
    energy_alignments JSONB DEFAULT '{}', -- elemental affinities
    privacy_level VARCHAR(20) DEFAULT 'private', -- private, community, public
    sacred_path VARCHAR(100), -- jungian, shamanic, hermetic, etc.
    created_at TIMESTAMP DEFAULT NOW()
);

-- Symbolic states with archetypal data
CREATE TABLE archetypal_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    practitioner_id UUID NOT NULL REFERENCES practitioners(id) ON DELETE CASCADE,
    state_data JSONB NOT NULL, -- full symbolic state
    archetypes JSONB NOT NULL, -- active archetypes and their strength
    energies JSONB NOT NULL, -- elemental energy levels
    integrations JSONB DEFAULT '[]', -- completed integrations
    symbols JSONB DEFAULT '[]', -- emergent symbols
    transformations JSONB DEFAULT '[]', -- active processes
    state_hash VARCHAR(64), -- cryptographic state verification
    created_at TIMESTAMP DEFAULT NOW()
);

-- Ritual definitions and WASM modules
CREATE TABLE sacred_rituals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    intent TEXT NOT NULL, -- ritual purpose
    tradition VARCHAR(100), -- jungian, shamanic, alchemical, etc.
    difficulty_level VARCHAR(20) DEFAULT 'beginner',
    required_archetypes JSONB DEFAULT '[]',
    energy_requirements JSONB DEFAULT '{}',
    wasm_module_data BYTEA, -- WASM binary data
    wasm_module_hash VARCHAR(64), -- verification hash
    module_language VARCHAR(50), -- rust, c, assemblyscript, etc.
    author_id UUID REFERENCES practitioners(id),
    usage_count INTEGER DEFAULT 0,
    effectiveness_rating DECIMAL(3,2) DEFAULT 0.0,
    is_public BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Ritual execution history
CREATE TABLE ritual_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    practitioner_id UUID NOT NULL REFERENCES practitioners(id) ON DELETE CASCADE,
    ritual_id UUID NOT NULL REFERENCES sacred_rituals(id),
    pre_state_id UUID REFERENCES archetypal_states(id),
    post_state_id UUID REFERENCES archetypal_states(id),
    execution_duration_ms INTEGER,
    transformation_intensity DECIMAL(3,2), -- how much state changed
    subjective_experience TEXT, -- practitioner's notes
    ai_interpretation TEXT, -- AI oracle insights
    integration_notes TEXT, -- follow-up integration guidance
    effectiveness_rating INTEGER CHECK (effectiveness_rating >= 1 AND effectiveness_rating <= 10),
    created_at TIMESTAMP DEFAULT NOW()
);

-- AI reflections and archetypal interpretations
CREATE TABLE oracle_insights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID REFERENCES ritual_sessions(id) ON DELETE CASCADE,
    insight_type VARCHAR(50), -- interpretation, guidance, warning, celebration
    archetypal_analysis JSONB, -- detailed symbolic interpretation
    integration_suggestions JSONB, -- practical next steps
    symbolic_emergence JSONB, -- new symbols that appeared
    oracle_model VARCHAR(100), -- which AI model provided insight
    confidence_score DECIMAL(3,2),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Collective ritual spaces (Phase 4)
CREATE TABLE collective_spaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    purpose TEXT NOT NULL,
    tradition VARCHAR(100),
    facilitator_id UUID REFERENCES practitioners(id),
    participant_limit INTEGER DEFAULT 12, -- sacred number
    current_participants INTEGER DEFAULT 0,
    shared_state JSONB DEFAULT '{}', -- collective symbolic state
    ritual_schedule JSONB, -- when collective rituals occur
    access_level VARCHAR(20) DEFAULT 'invite', -- open, invite, private
    created_at TIMESTAMP DEFAULT NOW()
);
```

---

## **WASM Ritual Runtime Engine**

```rust
use axum::{extract::State, Json};
use wasmtime::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchetypalState {
    pub archetypes: HashMap<String, f64>, // archetype name -> activation strength
    pub energies: HashMap<String, f64>,   // energy type -> current level
    pub integrations: Vec<String>,        // completed integration work
    pub symbols: Vec<String>,             // active symbols
    pub transformations: Vec<String>,     // ongoing processes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RitualExecution {
    pub ritual_id: uuid::Uuid,
    pub practitioner_state: ArchetypalState,
    pub ritual_parameters: HashMap<String, serde_json::Value>,
    pub intention: String,
}

#[derive(Debug, Serialize)]
pub struct TransformationResult {
    pub new_state: ArchetypalState,
    pub transformation_intensity: f64,
    pub emerged_symbols: Vec<String>,
    pub integration_required: Vec<String>,
    pub next_rituals_suggested: Vec<String>,
    pub oracle_consultation_recommended: bool,
}

pub struct SacredWasmRuntime {
    engine: Engine,
    ritual_modules: HashMap<uuid::Uuid, Module>,
}

impl SacredWasmRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let engine = Engine::default();
        Ok(Self {
            engine,
            ritual_modules: HashMap::new(),
        })
    }
    
    pub async fn load_ritual_module(&mut self, ritual_id: uuid::Uuid, wasm_bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let module = Module::from_binary(&self.engine, wasm_bytes)?;
        self.ritual_modules.insert(ritual_id, module);
        Ok(())
    }
    
    pub async fn execute_ritual(&self, execution: RitualExecution) -> Result<TransformationResult, Box<dyn std::error::Error>> {
        let module = self.ritual_modules.get(&execution.ritual_id)
            .ok_or("Ritual module not loaded")?;
        
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, module, &[])?;
        
        // Get WASM exports
        let ritual_func = instance
            .get_typed_func::<(), i32>(&mut store, "execute_ritual")?;
        
        let state_func = instance
            .get_typed_func::<(), i32>(&mut store, "get_state_transformation")
            .ok(); // Optional function
        
        // Prepare ritual context in WASM memory
        self.setup_ritual_context(&mut store, &instance, &execution)?;
        
        // Execute the sacred ritual
        let execution_start = std::time::Instant::now();
        let result_code = ritual_func.call(&mut store, ())?;
        let execution_duration = execution_start.elapsed();
        
        if result_code != 0 {
            return Err(format!("Ritual execution failed with code: {}", result_code).into());
        }
        
        // Extract transformation results
        let transformation = self.extract_transformation_results(&mut store, &instance, execution.practitioner_state)?;
        
        // Calculate transformation intensity
        let intensity = self.calculate_transformation_intensity(&execution.practitioner_state, &transformation.new_state);
        
        Ok(TransformationResult {
            new_state: transformation.new_state,
            transformation_intensity: intensity,
            emerged_symbols: transformation.emerged_symbols,
            integration_required: transformation.integration_required,
            next_rituals_suggested: self.suggest_next_rituals(&transformation.new_state),
            oracle_consultation_recommended: intensity > 0.7 || !transformation.emerged_symbols.is_empty(),
        })
    }
    
    fn setup_ritual_context(&self, store: &mut Store<()>, instance: &Instance, execution: &RitualExecution) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize archetypal state to WASM memory
        let state_json = serde_json::to_string(&execution.practitioner_state)?;
        let params_json = serde_json::to_string(&execution.ritual_parameters)?;
        
        // Write to WASM memory (simplified - real implementation would use proper memory management)
        if let Some(memory) = instance.get_memory(&mut *store, "memory") {
            let data = memory.data_mut(&mut *store);
            
            // Write state at offset 0
            let state_bytes = state_json.as_bytes();
            if state_bytes.len() < 1024 {
                data[0..state_bytes.len()].copy_from_slice(state_bytes);
            }
            
            // Write parameters at offset 1024
            let param_bytes = params_json.as_bytes();
            if param_bytes.len() < 1024 {
                data[1024..1024 + param_bytes.len()].copy_from_slice(param_bytes);
            }
        }
        
        Ok(())
    }
    
    fn calculate_transformation_intensity(&self, old_state: &ArchetypalState, new_state: &ArchetypalState) -> f64 {
        let mut total_change = 0.0;
        let mut change_count = 0;
        
        // Calculate archetype activation changes
        for (archetype, &old_value) in &old_state.archetypes {
            if let Some(&new_value) = new_state.archetypes.get(archetype) {
                total_change += (new_value - old_value).abs();
                change_count += 1;
            }
        }
        
        // Calculate energy level changes
        for (energy, &old_value) in &old_state.energies {
            if let Some(&new_value) = new_state.energies.get(energy) {
                total_change += (new_value - old_value).abs();
                change_count += 1;
            }
        }
        
        // Account for new integrations and symbols
        let integration_change = (new_state.integrations.len() as f64 - old_state.integrations.len() as f64) * 0.2;
        let symbol_change = (new_state.symbols.len() as f64 - old_state.symbols.len() as f64) * 0.3;
        
        if change_count > 0 {
            (total_change / change_count as f64) + integration_change + symbol_change
        } else {
            0.0
        }
    }
    
    fn suggest_next_rituals(&self, current_state: &ArchetypalState) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Suggest integration rituals for high-energy states
        let total_energy: f64 = current_state.energies.values().sum();
        if total_energy > 5.0 {
            suggestions.push("energy_integration".to_string());
        }
        
        // Suggest shadow work for imbalanced archetypes
        if let (Some(&light), Some(&shadow)) = (current_state.archetypes.get("Light"), current_state.archetypes.get("Shadow")) {
            if (light - shadow).abs() > 0.5 {
                suggestions.push("shadow_integration".to_string());
            }
        }
        
        // Suggest completion rituals for unfinished transformations
        if current_state.transformations.len() > 2 {
            suggestions.push("transformation_completion".to_string());
        }
        
        suggestions
    }
}
```

---

## **AI Oracle Integration**

```rust
use serde_json::json;
use reqwest::Client;

pub struct ArchetypalOracle {
    client: Client,
    api_key: String,
    model: String, // "anthropic/claude-3-haiku" or "openai/gpt-4"
}

impl ArchetypalOracle {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
    
    pub async fn interpret_transformation(&self, session: &RitualSession, pre_state: &ArchetypalState, post_state: &ArchetypalState) -> Result<OracleInsight, Box<dyn std::error::Error>> {
        let archetypal_context = self.build_archetypal_context(pre_state, post_state);
        
        let prompt = format!(r#"
Sacred Oracle Interpretation Request:

A practitioner has completed the ritual "{}" with the intention: "{}"

PRE-RITUAL ARCHETYPAL STATE:
{}

POST-RITUAL ARCHETYPAL STATE:
{}

TRANSFORMATION INTENSITY: {:.2}

As an archetypal oracle versed in Jungian depth psychology, shamanic wisdom, and sacred transformation, provide:

1. SYMBOLIC INTERPRETATION: What does this transformation represent in archetypal terms?
2. INTEGRATION GUIDANCE: What practices will help embody this change?
3. EMERGENT SYMBOLS: What new symbols or meanings are arising?
4. SHADOW DYNAMICS: What shadow material might be activated or integrated?
5. NEXT STEPS: What rituals or practices would support continued growth?
6. WARNINGS: Any cautionary guidance for this transformation intensity?

Respond with wisdom, compassion, and reverence for the sacred journey.
"#, 
            session.ritual_name,
            session.intention,
            serde_json::to_string_pretty(pre_state)?,
            serde_json::to_string_pretty(post_state)?,
            session.transformation_intensity
        );
        
        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://control-engine.sacred.dev")
            .json(&json!({
                "model": self.model,
                "messages": [
                    {"role": "system", "content": "You are a wise archetypal oracle, versed in Jungian psychology, shamanic wisdom, and sacred transformation practices. You interpret symbolic states and transformations with depth, compassion, and practical guidance."},
                    {"role": "user", "content": prompt}
                ],
                "temperature": 0.8,
                "max_tokens": 1000
            }))
            .send()
            .await?;
        
        let oracle_response: serde_json::Value = response.json().await?;
        let interpretation = oracle_response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Oracle consultation unavailable")
            .to_string();
        
        // Parse structured insights from the response
        let insights = self.parse_oracle_insights(&interpretation);
        
        Ok(OracleInsight {
            interpretation,
            symbolic_analysis: insights.symbolic_analysis,
            integration_guidance: insights.integration_guidance,
            emergent_symbols: insights.emergent_symbols,
            next_rituals: insights.next_rituals,
            cautions: insights.cautions,
            confidence_score: 0.85, // High confidence for AI oracle insights
        })
    }
    
    fn build_archetypal_context(&self, pre_state: &ArchetypalState, post_state: &ArchetypalState) -> String {
        let mut context = String::new();
        
        // Describe archetype changes
        context.push_str("ARCHETYPAL SHIFTS:\n");
        for (archetype, &pre_value) in &pre_state.archetypes {
            if let Some(&post_value) = post_state.archetypes.get(archetype) {
                let change = post_value - pre_value;
                if change.abs() > 0.1 {
                    context.push_str(&format!("  {}: {:.2} → {:.2} ({}{})\n", 
                        archetype, pre_value, post_value, 
                        if change > 0 { "+" } else { "" }, change
                    ));
                }
            }
        }
        
        // Describe energy changes
        context.push_str("\nENERGETIC SHIFTS:\n");
        for (energy, &pre_value) in &pre_state.energies {
            if let Some(&post_value) = post_state.energies.get(energy) {
                let change = post_value - pre_value;
                if change.abs() > 0.1 {
                    context.push_str(&format!("  {} Energy: {:.2} → {:.2} ({}{})\n", 
                        energy, pre_value, post_value,
                        if change > 0 { "+" } else { "" }, change
                    ));
                }
            }
        }
        
        // Note new integrations and symbols
        let new_integrations: Vec<_> = post_state.integrations.iter()
            .filter(|i| !pre_state.integrations.contains(i))
            .collect();
        if !new_integrations.is_empty() {
            context.push_str(&format!("\nNEW INTEGRATIONS: {:?}\n", new_integrations));
        }
        
        let new_symbols: Vec<_> = post_state.symbols.iter()
            .filter(|s| !pre_state.symbols.contains(s))
            .collect();
        if !new_symbols.is_empty() {
            context.push_str(&format!("EMERGED SYMBOLS: {:?}\n", new_symbols));
        }
        
        context
    }
}
```

---

## **Deployment Strategy**

```bash
# Rust + WASM deployment to Fly.io
fly launch --name control-engine-api
fly postgres create sacred-state-db
fly secrets set DATABASE_URL=postgresql://...
fly secrets set OPENROUTER_API_KEY=...
fly secrets set SACRED_ENCRYPTION_KEY=...
fly deploy

# Environment variables
DATABASE_URL=postgresql://sacred-state-db.internal:5432/sacred
OPENROUTER_API_KEY=sk-or-...
SACRED_ENCRYPTION_KEY=32-byte-hex-key-for-state-encryption
RUST_LOG=info
WASM_MAX_MEMORY=268435456 # 256MB max per ritual
```

---

## **Development Timeline**

**Week 1**: Core Rust API, database schema, user authentication
**Week 2**: WASM runtime integration, ritual execution engine
**Week 3**: AI oracle system, transformation analysis
**Week 4**: Web interface, ritual sharing, deployment
**Week 5**: Collective spaces foundation (Phase 4)

**Estimated Development**: 5-6 weeks to sacred web platform