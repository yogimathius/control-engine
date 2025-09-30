use crate::{CodexError, SymbolicState};
use chrono::{DateTime, Utc};
use rand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;
use wasmtime::*;

/// Represents the outcome of a ritual execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualResult {
    pub ritual_name: String,
    pub execution_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
    pub symbolic_outputs: HashMap<String, serde_json::Value>,
    pub state_changes: Vec<StateChange>,
    pub emergent_symbols: Vec<String>,
    pub completion_status: CompletionStatus,
    pub resonance_level: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionStatus {
    Complete,
    PartialIntegration,
    Interrupted,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub change_type: ChangeType,
    pub description: String,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    ArchetypeActivation,
    EnergyShift,
    Integration,
    SymbolResolution,
    Transformation,
}

/// Defines the structure and behavior of a ritual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualDefinition {
    pub name: String,
    pub description: String,
    pub intent: String,
    pub required_archetypes: Vec<String>,
    pub energy_requirements: HashMap<String, f64>,
    pub wasm_module_path: Option<String>,
    pub native_handler: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// The ritual execution engine
pub struct Ritual {
    pub definition: RitualDefinition,
    wasm_engine: Option<Engine>,
    wasm_module: Option<Module>,
}

impl Ritual {
    pub fn new(definition: RitualDefinition) -> Self {
        Self {
            definition,
            wasm_engine: None,
            wasm_module: None,
        }
    }

    pub fn load_wasm_module(&mut self) -> Result<(), CodexError> {
        if let Some(module_path) = &self.definition.wasm_module_path {
            let engine = Engine::default();
            let module_bytes = std::fs::read(module_path)?;
            let module = Module::new(&engine, &module_bytes)?;

            self.wasm_engine = Some(engine);
            self.wasm_module = Some(module);
        }
        Ok(())
    }

    pub fn load_wasm_module_from_bytes(&mut self, wasm_data: &[u8]) -> Result<(), CodexError> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_data)?;

        self.wasm_engine = Some(engine);
        self.wasm_module = Some(module);
        Ok(())
    }

    pub async fn execute(&self, state: &mut SymbolicState) -> Result<RitualResult, CodexError> {
        let start_time = std::time::Instant::now();
        let execution_id = Uuid::new_v4();

        // Try WASM execution first, then fall back to native
        let mut result = if self.wasm_engine.is_some() && self.wasm_module.is_some() {
            self.execute_wasm_ritual(state, execution_id).await
                .unwrap_or_else(|e| {
                    tracing::warn!("WASM execution failed, falling back to native: {}", e);
                    self.execute_native_ritual(state, execution_id)
                })
        } else {
            self.execute_native_ritual(state, execution_id)
        };

        let duration = start_time.elapsed();
        result.duration_ms = duration.as_millis() as u64;

        Ok(result)
    }

    async fn execute_wasm_ritual(&self, state: &mut SymbolicState, execution_id: Uuid) -> Result<RitualResult, CodexError> {
        let engine = self.wasm_engine.as_ref().ok_or(CodexError::WasmExecution { error: "No WASM engine".to_string() })?;
        let module = self.wasm_module.as_ref().ok_or(CodexError::WasmExecution { error: "No WASM module".to_string() })?;

        // Create a store and instantiate the module
        let mut store = Store::new(engine, ());
        
        // Create linker for host functions
        let mut linker = Linker::new(engine);
        linker.func_wrap("codex", "log", |_: i32, _: i32| {
            tracing::info!("WASM ritual executing");
        })?;
        linker.func_wrap("codex", "get_archetype_activation", |_: i32, _: i32| -> f64 {
            0.5 // Placeholder
        })?;
        linker.func_wrap("codex", "set_archetype_activation", |_: i32, _: i32, _: f64| {
            // Placeholder
        })?;
        linker.func_wrap("codex", "add_symbol", |_: i32, _: i32| {
            // Placeholder
        })?;
        linker.func_wrap("codex", "get_random", || -> f64 {
            rand::random::<f64>()
        })?;

        let instance = linker.instantiate(&mut store, module)?;
        
        // Get the execute_ritual function
        let execute_func = instance
            .get_typed_func::<(), i32>(&mut store, "execute_ritual")
            .map_err(|e| CodexError::WasmExecution { error: format!("Failed to get execute_ritual function: {}", e) })?;

        // Execute the ritual
        let result_code = execute_func.call(&mut store, ())?;
        
        // Get resonance if available
        let resonance = if let Ok(resonance_func) = instance.get_typed_func::<(), f64>(&mut store, "get_resonance") {
            resonance_func.call(&mut store, ()).unwrap_or(0.5)
        } else {
            0.5
        };

        // Create result based on WASM execution
        let result = RitualResult {
            ritual_name: self.definition.name.clone(),
            execution_id,
            timestamp: chrono::Utc::now(),
            duration_ms: 0, // Will be set by caller
            symbolic_outputs: std::collections::HashMap::new(),
            state_changes: vec![StateChange {
                change_type: ChangeType::Transformation,
                description: "WASM ritual executed successfully".to_string(),
                magnitude: resonance,
            }],
            emergent_symbols: vec!["🔮".to_string(), "∿".to_string()],
            completion_status: if result_code == 0 { 
                CompletionStatus::Complete 
            } else { 
                CompletionStatus::Error(format!("WASM returned code: {}", result_code)) 
            },
            resonance_level: resonance,
        };

        Ok(result)
    }

    fn execute_native_ritual(&self, state: &mut SymbolicState, execution_id: Uuid) -> RitualResult {
        let start_time = Instant::now();
        state.begin_transformation(format!("ritual:{}", self.definition.name));

        let mut result = RitualResult {
            ritual_name: self.definition.name.clone(),
            execution_id,
            timestamp: Utc::now(),
            duration_ms: 0,
            symbolic_outputs: HashMap::new(),
            state_changes: Vec::new(),
            emergent_symbols: Vec::new(),
            completion_status: CompletionStatus::Complete,
            resonance_level: 0.0,
        };

        // Check archetype prerequisites
        let archetype_resonance = self.check_archetype_prerequisites(state);
        if archetype_resonance < 0.3 {
            result.completion_status = CompletionStatus::PartialIntegration;
        }

        // Execute basic ritual transformations
        match self.definition.name.as_str() {
            "shadow_integration" => {
                self.execute_shadow_integration(state, &mut result);
            }
            "energy_attunement" => {
                self.execute_energy_attunement(state, &mut result);
            }
            _ => {
                // Generic ritual execution
                result.resonance_level = archetype_resonance * 0.8;
                result.emergent_symbols.push("✨".to_string());
            }
        }

        // Calculate final resonance
        result.resonance_level = self.calculate_resonance(state, archetype_resonance);

        // Complete the transformation
        state.complete_transformation(&format!("ritual:{}", self.definition.name));

        result.duration_ms = start_time.elapsed().as_millis() as u64;
        result
    }

    fn execute_shadow_integration(&self, state: &mut SymbolicState, result: &mut RitualResult) {
        // Shadow integration logic
        let shadow_activation = state.archetypes.get("Shadow").map(|a| a.activation_level).unwrap_or(0.0);
        let integration_factor = 0.2 + (rand::random::<f64>() * 0.3);
        
        // Increase shadow awareness
        if let Some(shadow_arch) = state.archetypes.get_mut("Shadow") {
            shadow_arch.activation_level = (shadow_arch.activation_level + integration_factor).min(1.0);
        }
        
        result.emergent_symbols = vec!["◯●◯".to_string(), "🌑".to_string()];
        result.resonance_level = (shadow_activation + integration_factor) * 0.7;
    }
    
    fn execute_energy_attunement(&self, state: &mut SymbolicState, result: &mut RitualResult) {
        // Energy balancing logic
        let energy_names = ["Fire", "Water", "Earth", "Air"];
        let total_energy: f64 = energy_names.iter()
            .map(|name| state.energies.get(*name).map(|e| e.amplitude).unwrap_or(0.0))
            .sum();
        
        let target_level = total_energy / 4.0;
        let adjustment = 0.3;
        
        // Adjust energies toward balance
        for energy_name in &energy_names {
            if let Some(energy) = state.energies.get_mut(*energy_name) {
                energy.amplitude = energy.amplitude + (target_level - energy.amplitude) * adjustment;
            }
        }
        
        result.emergent_symbols = vec!["∿∿∿".to_string(), "⚡".to_string()];
        result.resonance_level = 0.8;
    }

    fn check_archetype_prerequisites(&self, state: &SymbolicState) -> f64 {
        let mut total_resonance = 0.0;
        let mut count = 0;

        for archetype_name in &self.definition.required_archetypes {
            if let Some(archetype) = state.archetypes.get(archetype_name) {
                total_resonance += archetype.activation_level;
                count += 1;
            }
        }

        if count > 0 {
            total_resonance / count as f64
        } else {
            0.5 // Default resonance if no prerequisites
        }
    }

    fn calculate_resonance(&self, state: &SymbolicState, base_resonance: f64) -> f64 {
        // Calculate resonance based on energy alignment and archetype activation
        let energy_alignment = self.calculate_energy_alignment(state);
        let symbol_coherence = self.calculate_symbol_coherence(state);
        
        (base_resonance * 0.4 + energy_alignment * 0.3 + symbol_coherence * 0.3).min(1.0)
    }

    fn calculate_energy_alignment(&self, state: &SymbolicState) -> f64 {
        let mut alignment_score = 0.0;
        let mut count = 0;

        for (energy_name, required_level) in &self.definition.energy_requirements {
            if let Some(energy) = state.energies.get(energy_name) {
                let alignment = 1.0 - (energy.amplitude - required_level).abs();
                alignment_score += alignment.max(0.0);
                count += 1;
            }
        }

        if count > 0 {
            alignment_score / count as f64
        } else {
            0.7 // Default alignment if no energy requirements
        }
    }

    fn calculate_symbol_coherence(&self, state: &SymbolicState) -> f64 {
        // Simple coherence calculation based on unresolved symbols
        let unresolved_ratio = if state.unresolved_symbols.is_empty() {
            0.0
        } else {
            state.unresolved_symbols.len() as f64 / (state.unresolved_symbols.len() + 1) as f64
        };
        
        1.0 - unresolved_ratio.min(0.8)
    }
}
