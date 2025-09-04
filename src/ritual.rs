use crate::{CodexError, SymbolicState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

        // Begin the ritual transformation
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

        // Execute ritual logic
        match self.execute_ritual_logic(state, &mut result).await {
            Ok(_) => {
                // Calculate final resonance
                result.resonance_level = self.calculate_resonance(state, archetype_resonance);

                // Complete the transformation
                state.complete_transformation(&format!("ritual:{}", self.definition.name));
            }
            Err(e) => {
                result.completion_status = CompletionStatus::Error(e.to_string());
                return Err(e);
            }
        }

        result.duration_ms = start_time.elapsed().as_millis() as u64;
        Ok(result)
    }

    async fn execute_ritual_logic(
        &self,
        state: &mut SymbolicState,
        result: &mut RitualResult,
    ) -> Result<(), CodexError> {
        // Execute WASM module if available
        if let (Some(engine), Some(module)) = (&self.wasm_engine, &self.wasm_module) {
            self.execute_wasm_ritual(engine, module, state, result)
                .await?;
        } else {
            // Execute native ritual handler
            self.execute_native_ritual(state, result).await?;
        }

        Ok(())
    }

    async fn execute_wasm_ritual(
        &self,
        engine: &Engine,
        module: &Module,
        state: &mut SymbolicState,
        result: &mut RitualResult,
    ) -> Result<(), CodexError> {
        let mut store = Store::new(engine, ());
        let instance = Instance::new(&mut store, module, &[])?;

        // Get the ritual function from WASM module
        let ritual_func = instance
            .get_typed_func::<(), i32>(&mut store, "execute_ritual")
            .map_err(|e| CodexError::WasmExecution {
                error: e.to_string(),
            })?;

        // Execute the ritual function
        let wasm_result =
            ritual_func
                .call(&mut store, ())
                .map_err(|e| CodexError::WasmExecution {
                    error: e.to_string(),
                })?;

        // Process WASM result
        self.process_wasm_result(wasm_result, state, result);

        Ok(())
    }

    async fn execute_native_ritual(
        &self,
        state: &mut SymbolicState,
        result: &mut RitualResult,
    ) -> Result<(), CodexError> {
        // Execute built-in ritual based on handler name
        match self.definition.native_handler.as_deref() {
            Some("shadow_integration") => self.shadow_integration_ritual(state, result),
            Some("energy_attunement") => self.energy_attunement_ritual(state, result),
            Some("archetype_invocation") => self.archetype_invocation_ritual(state, result),
            Some("void_contemplation") => self.void_contemplation_ritual(state, result),
            _ => self.default_symbolic_ritual(state, result),
        }

        Ok(())
    }

    fn shadow_integration_ritual(&self, state: &mut SymbolicState, result: &mut RitualResult) {
        // Find shadow aspects to integrate
        let mut shadows_integrated = 0;

        for archetype in state.archetypes.values_mut() {
            if !archetype.shadow_aspects.is_empty() && archetype.activation_level > 0.5 {
                if let Some(shadow) = archetype.shadow_aspects.pop() {
                    archetype
                        .light_aspects
                        .push(format!("Integrated: {}", shadow));
                    shadows_integrated += 1;

                    result.state_changes.push(StateChange {
                        change_type: ChangeType::Integration,
                        description: format!(
                            "Integrated shadow aspect '{}' of {}",
                            shadow, archetype.name
                        ),
                        magnitude: 0.7,
                    });
                }
            }
        }

        result.symbolic_outputs.insert(
            "shadows_integrated".to_string(),
            serde_json::json!(shadows_integrated),
        );

        if shadows_integrated > 0 {
            result.emergent_symbols.push("🌑→🌕".to_string()); // Shadow to light
            result.emergent_symbols.push("∫∂∇".to_string()); // Integration symbol
        }
    }

    fn energy_attunement_ritual(&self, state: &mut SymbolicState, result: &mut RitualResult) {
        // Attune energies to harmonious frequencies
        let target_frequency = 7.83; // Schumann resonance
        let mut energies_attuned = 0;

        for energy in state.energies.values_mut() {
            let frequency_diff = (energy.frequency - target_frequency).abs();
            if frequency_diff > 1.0 {
                energy.modulate((target_frequency - energy.frequency) * 0.3, 0.1);
                energies_attuned += 1;

                result.state_changes.push(StateChange {
                    change_type: ChangeType::EnergyShift,
                    description: format!("Attuned {} to harmonic frequency", energy.name),
                    magnitude: 0.5,
                });
            }
        }

        result.symbolic_outputs.insert(
            "energies_attuned".to_string(),
            serde_json::json!(energies_attuned),
        );

        if energies_attuned > 0 {
            result.emergent_symbols.push("∿∿∿".to_string()); // Harmonic waves
            result.emergent_symbols.push("⚡→⚡".to_string()); // Energy transformation
        }
    }

    fn archetype_invocation_ritual(&self, state: &mut SymbolicState, result: &mut RitualResult) {
        // Invoke required archetypes with higher intensity
        for archetype_name in &self.definition.required_archetypes {
            if let Some(archetype) = state.archetypes.get_mut(archetype_name) {
                archetype.invoke(0.3);

                result.state_changes.push(StateChange {
                    change_type: ChangeType::ArchetypeActivation,
                    description: format!("Invoked archetype: {}", archetype_name),
                    magnitude: 0.6,
                });
            }
        }

        result.emergent_symbols.push("🔯".to_string()); // Sacred geometry
        result.emergent_symbols.push("∆∇∆".to_string()); // Archetypal invocation
    }

    fn void_contemplation_ritual(&self, state: &mut SymbolicState, result: &mut RitualResult) {
        // A ritual of emptiness and potential
        state.add_unresolved_symbol("∅".to_string()); // Void symbol

        // Reduce all activations slightly to create space
        for archetype in state.archetypes.values_mut() {
            archetype.activation_level = (archetype.activation_level * 0.9).max(0.0);
        }

        result.state_changes.push(StateChange {
            change_type: ChangeType::Transformation,
            description: "Entered contemplative void state".to_string(),
            magnitude: 0.8,
        });

        result.emergent_symbols.push("∅".to_string()); // Void
        result.emergent_symbols.push("○".to_string()); // Emptiness/potential
        result.emergent_symbols.push("∞".to_string()); // Infinite possibility
    }

    fn default_symbolic_ritual(&self, state: &mut SymbolicState, result: &mut RitualResult) {
        // Generic symbolic processing
        let symbol = format!("◊{}", self.definition.name);
        state.add_unresolved_symbol(symbol.clone());

        result.emergent_symbols.push(symbol);
        result.state_changes.push(StateChange {
            change_type: ChangeType::SymbolResolution,
            description: format!("Generated symbolic marker for {}", self.definition.name),
            magnitude: 0.4,
        });
    }

    fn process_wasm_result(
        &self,
        wasm_result: i32,
        _state: &mut SymbolicState,
        result: &mut RitualResult,
    ) {
        // Interpret WASM return codes
        match wasm_result {
            0 => {
                result.emergent_symbols.push("✓".to_string());
            }
            1 => {
                result.emergent_symbols.push("⚡".to_string());
                // Energy activation
            }
            2 => {
                result.emergent_symbols.push("🔄".to_string());
                // Transformation cycle
            }
            _ => {
                result.emergent_symbols.push("?".to_string());
                // Unknown result
            }
        }
    }

    fn check_archetype_prerequisites(&self, state: &SymbolicState) -> f64 {
        if self.definition.required_archetypes.is_empty() {
            return 1.0;
        }

        let total_activation: f64 = self
            .definition
            .required_archetypes
            .iter()
            .map(|name| {
                state
                    .archetypes
                    .get(name)
                    .map(|a| a.activation_level)
                    .unwrap_or(0.0)
            })
            .sum();

        total_activation / self.definition.required_archetypes.len() as f64
    }

    fn calculate_resonance(&self, state: &SymbolicState, archetype_resonance: f64) -> f64 {
        let energy_coherence = self.calculate_energy_coherence(state);
        let integration_depth = self.calculate_integration_depth(state);

        (archetype_resonance + energy_coherence + integration_depth) / 3.0
    }

    fn calculate_energy_coherence(&self, state: &SymbolicState) -> f64 {
        if state.energies.is_empty() {
            return 0.5;
        }

        let avg_amplitude: f64 =
            state.energies.values().map(|e| e.amplitude).sum::<f64>() / state.energies.len() as f64;

        avg_amplitude
    }

    fn calculate_integration_depth(&self, state: &SymbolicState) -> f64 {
        if state.integrations.is_empty() {
            return 0.0;
        }

        let avg_depth: f64 = state
            .integrations
            .values()
            .map(|i| i.depth_level as f64 / 10.0)
            .sum::<f64>()
            / state.integrations.len() as f64;

        avg_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Archetype, Element, Energy, Integration};

    fn create_test_state() -> SymbolicState {
        let mut state = SymbolicState::new();

        // Add test archetypes
        let mut shadow = Archetype::new("Shadow".to_string(), "Dark aspects".to_string());
        shadow.activation_level = 0.6;
        shadow.shadow_aspects.push("Pride".to_string());
        shadow.shadow_aspects.push("Envy".to_string());
        state.add_archetype(shadow);

        let mut sage = Archetype::new("Sage".to_string(), "Wisdom".to_string());
        sage.activation_level = 0.8;
        state.add_archetype(sage);

        // Add test energies
        let mut fire = Energy::new("Fire".to_string(), 10.0, Element::Fire);
        fire.amplitude = 0.7;
        state.add_energy(fire);

        let mut water = Energy::new("Water".to_string(), 5.0, Element::Water);
        water.amplitude = 0.6;
        state.add_energy(water);

        // Add test integration
        state.add_integration(Integration::new(
            "Test Integration".to_string(),
            "Test wisdom".to_string(),
            vec![],
        ));

        state
    }

    fn create_test_ritual_definition(name: &str, handler: Option<&str>) -> RitualDefinition {
        let mut energy_requirements = HashMap::new();
        energy_requirements.insert("Fire".to_string(), 0.5);

        let mut required_archetypes = vec![];
        if handler == Some("archetype_invocation") {
            required_archetypes.push("Sage".to_string());
        }

        RitualDefinition {
            name: name.to_string(),
            description: format!("Test ritual: {}", name),
            intent: "Testing ritual execution".to_string(),
            required_archetypes,
            energy_requirements,
            wasm_module_path: None,
            native_handler: handler.map(String::from),
            parameters: HashMap::new(),
        }
    }

    #[test]
    fn test_ritual_creation() {
        let definition = create_test_ritual_definition("test_ritual", None);
        let ritual = Ritual::new(definition.clone());

        assert_eq!(ritual.definition.name, "test_ritual");
        assert_eq!(ritual.definition.description, "Test ritual: test_ritual");
        assert_eq!(ritual.definition.intent, "Testing ritual execution");
        assert!(ritual.wasm_engine.is_none());
        assert!(ritual.wasm_module.is_none());
    }

    #[tokio::test]
    async fn test_shadow_integration_ritual() {
        let definition = create_test_ritual_definition("shadow_work", Some("shadow_integration"));
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();

        // Verify initial state
        let shadow_archetype = state.archetypes.get("Shadow").unwrap();
        assert_eq!(shadow_archetype.shadow_aspects.len(), 2);
        assert_eq!(shadow_archetype.light_aspects.len(), 0);

        // Execute ritual
        let result = ritual.execute(&mut state).await.unwrap();

        // Verify execution results
        assert_eq!(result.ritual_name, "shadow_work");
        assert!(matches!(
            result.completion_status,
            CompletionStatus::Complete
        ));
        assert!(!result.state_changes.is_empty());
        assert!(result.emergent_symbols.contains(&"🌑→🌕".to_string()));
        assert!(result.emergent_symbols.contains(&"∫∂∇".to_string()));

        // Verify state changes
        let shadow_archetype = state.archetypes.get("Shadow").unwrap();
        assert_eq!(shadow_archetype.shadow_aspects.len(), 1); // One integrated
        assert_eq!(shadow_archetype.light_aspects.len(), 1); // One added to light

        // Verify symbolic outputs
        let shadows_integrated = result.symbolic_outputs.get("shadows_integrated").unwrap();
        assert_eq!(shadows_integrated.as_u64().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_energy_attunement_ritual() {
        let definition = create_test_ritual_definition("energy_tune", Some("energy_attunement"));
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();

        // Store initial frequencies
        let initial_fire_freq = state.energies.get("Fire").unwrap().frequency;
        let initial_water_freq = state.energies.get("Water").unwrap().frequency;

        // Execute ritual
        let result = ritual.execute(&mut state).await.unwrap();

        // Verify execution results
        assert_eq!(result.ritual_name, "energy_tune");
        assert!(matches!(
            result.completion_status,
            CompletionStatus::Complete
        ));
        assert!(result.emergent_symbols.contains(&"∿∿∿".to_string()));
        assert!(result.emergent_symbols.contains(&"⚡→⚡".to_string()));

        // Verify energy changes - frequencies should move toward 7.83
        let final_fire_freq = state.energies.get("Fire").unwrap().frequency;
        let final_water_freq = state.energies.get("Water").unwrap().frequency;

        // Fire frequency (10.0) should decrease toward 7.83
        assert!(final_fire_freq < initial_fire_freq);
        // Check it moved at least 25% toward target
        let expected_min_change = initial_fire_freq + (7.83 - initial_fire_freq) * 0.25;
        assert!(final_fire_freq <= expected_min_change);

        // Water frequency (5.0) should increase toward 7.83
        assert!(final_water_freq > initial_water_freq);
        assert!(final_water_freq < initial_water_freq + (7.83 - initial_water_freq) * 0.35);

        // Verify symbolic outputs
        let energies_attuned = result.symbolic_outputs.get("energies_attuned").unwrap();
        assert_eq!(energies_attuned.as_u64().unwrap(), 2);
    }

    #[tokio::test]
    async fn test_archetype_invocation_ritual() {
        let definition = create_test_ritual_definition("invoke", Some("archetype_invocation"));
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();

        // Store initial activation level
        let initial_activation = state.archetypes.get("Sage").unwrap().activation_level;

        // Execute ritual
        let result = ritual.execute(&mut state).await.unwrap();

        // Verify execution results
        assert_eq!(result.ritual_name, "invoke");
        assert!(matches!(
            result.completion_status,
            CompletionStatus::Complete
        ));
        assert!(result.emergent_symbols.contains(&"🔯".to_string()));
        assert!(result.emergent_symbols.contains(&"∆∇∆".to_string()));

        // Verify archetype activation increased
        let final_activation = state.archetypes.get("Sage").unwrap().activation_level;
        assert!(final_activation > initial_activation);
        assert_eq!(final_activation, (initial_activation + 0.3).min(1.0));

        // Verify state changes
        assert!(!result.state_changes.is_empty());
        let state_change = &result.state_changes[0];
        assert!(matches!(
            state_change.change_type,
            ChangeType::ArchetypeActivation
        ));
        assert!(state_change.description.contains("Invoked archetype: Sage"));
    }

    #[tokio::test]
    async fn test_void_contemplation_ritual() {
        let definition = create_test_ritual_definition("void", Some("void_contemplation"));
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();

        // Store initial activation levels
        let initial_shadow_activation = state.archetypes.get("Shadow").unwrap().activation_level;
        let initial_sage_activation = state.archetypes.get("Sage").unwrap().activation_level;

        // Execute ritual
        let result = ritual.execute(&mut state).await.unwrap();

        // Verify execution results
        assert_eq!(result.ritual_name, "void");
        assert!(matches!(
            result.completion_status,
            CompletionStatus::Complete
        ));
        assert!(result.emergent_symbols.contains(&"∅".to_string()));
        assert!(result.emergent_symbols.contains(&"○".to_string()));
        assert!(result.emergent_symbols.contains(&"∞".to_string()));

        // Verify void symbol was added
        assert!(state.unresolved_symbols.contains(&"∅".to_string()));

        // Verify activation levels reduced
        let final_shadow_activation = state.archetypes.get("Shadow").unwrap().activation_level;
        let final_sage_activation = state.archetypes.get("Sage").unwrap().activation_level;

        assert!(final_shadow_activation < initial_shadow_activation);
        assert!(final_sage_activation < initial_sage_activation);
        assert_eq!(final_shadow_activation, initial_shadow_activation * 0.9);
        assert_eq!(final_sage_activation, initial_sage_activation * 0.9);
    }

    #[tokio::test]
    async fn test_default_symbolic_ritual() {
        let definition = create_test_ritual_definition("custom", None);
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();

        // Execute ritual
        let result = ritual.execute(&mut state).await.unwrap();

        // Verify execution results
        assert_eq!(result.ritual_name, "custom");
        assert!(matches!(
            result.completion_status,
            CompletionStatus::Complete
        ));

        // Verify symbol was created and added
        let expected_symbol = "◊custom";
        assert!(result
            .emergent_symbols
            .contains(&expected_symbol.to_string()));
        assert!(state
            .unresolved_symbols
            .contains(&expected_symbol.to_string()));

        // Verify state change
        assert_eq!(result.state_changes.len(), 1);
        let state_change = &result.state_changes[0];
        assert!(matches!(
            state_change.change_type,
            ChangeType::SymbolResolution
        ));
        assert!(state_change
            .description
            .contains("Generated symbolic marker for custom"));
        assert_eq!(state_change.magnitude, 0.4);
    }

    #[test]
    fn test_check_archetype_prerequisites() {
        // Test with no requirements
        let definition = create_test_ritual_definition("no_reqs", None);
        let ritual = Ritual::new(definition);
        let state = create_test_state();

        let resonance = ritual.check_archetype_prerequisites(&state);
        assert_eq!(resonance, 1.0);

        // Test with existing archetype
        let mut definition = create_test_ritual_definition("with_reqs", None);
        definition.required_archetypes = vec!["Sage".to_string()];
        let ritual = Ritual::new(definition);

        let resonance = ritual.check_archetype_prerequisites(&state);
        assert_eq!(resonance, 0.8); // Sage has 0.8 activation

        // Test with missing archetype
        let mut definition = create_test_ritual_definition("missing_arch", None);
        definition.required_archetypes = vec!["NonExistent".to_string()];
        let ritual = Ritual::new(definition);

        let resonance = ritual.check_archetype_prerequisites(&state);
        assert_eq!(resonance, 0.0);

        // Test with multiple archetypes
        let mut definition = create_test_ritual_definition("multi_arch", None);
        definition.required_archetypes = vec!["Sage".to_string(), "Shadow".to_string()];
        let ritual = Ritual::new(definition);

        let resonance = ritual.check_archetype_prerequisites(&state);
        assert_eq!(resonance, (0.8 + 0.6) / 2.0); // Average of Sage (0.8) and Shadow (0.6)
    }

    #[test]
    fn test_calculate_energy_coherence() {
        let definition = create_test_ritual_definition("test", None);
        let ritual = Ritual::new(definition);
        let state = create_test_state();

        // Fire: 0.7, Water: 0.6, average = 0.65
        let coherence = ritual.calculate_energy_coherence(&state);
        assert!((coherence - 0.65).abs() < f64::EPSILON);

        // Test with empty energies
        let empty_state = SymbolicState::new();
        let coherence = ritual.calculate_energy_coherence(&empty_state);
        assert_eq!(coherence, 0.5);
    }

    #[test]
    fn test_calculate_integration_depth() {
        let definition = create_test_ritual_definition("test", None);
        let ritual = Ritual::new(definition);
        let state = create_test_state();

        // One integration with depth level 1, normalized to 0.1
        let depth = ritual.calculate_integration_depth(&state);
        assert_eq!(depth, 0.1);

        // Test with empty integrations
        let empty_state = SymbolicState::new();
        let depth = ritual.calculate_integration_depth(&empty_state);
        assert_eq!(depth, 0.0);
    }

    #[test]
    fn test_calculate_resonance() {
        let definition = create_test_ritual_definition("test", None);
        let ritual = Ritual::new(definition);
        let state = create_test_state();

        let archetype_resonance = 0.7;
        let resonance = ritual.calculate_resonance(&state, archetype_resonance);

        // Should be average of archetype_resonance (0.7), energy_coherence (0.65), integration_depth (0.1)
        let expected = (0.7 + 0.65 + 0.1) / 3.0;
        assert!((resonance - expected).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_ritual_transformation_tracking() {
        let definition =
            create_test_ritual_definition("transform_test", Some("shadow_integration"));
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();

        // Verify no active transformations initially
        assert!(state.active_transformations.is_empty());
        let initial_cycle = state.evolution_cycle;

        // Execute ritual
        let result = ritual.execute(&mut state).await.unwrap();

        // Verify transformation was completed
        assert!(state.active_transformations.is_empty());
        assert_eq!(state.evolution_cycle, initial_cycle + 1);
        assert!(matches!(
            result.completion_status,
            CompletionStatus::Complete
        ));
    }

    #[tokio::test]
    async fn test_partial_integration_status() {
        // Create ritual requiring high archetype activation
        let mut definition =
            create_test_ritual_definition("high_req", Some("archetype_invocation"));
        definition.required_archetypes = vec!["NonExistent".to_string()];
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();

        // Execute ritual with insufficient archetype activation
        let result = ritual.execute(&mut state).await.unwrap();

        // Should result in partial integration due to low archetype resonance
        assert!(matches!(
            result.completion_status,
            CompletionStatus::PartialIntegration
        ));
        assert!(result.resonance_level >= 0.0);
    }

    #[test]
    fn test_process_wasm_result() {
        let definition = create_test_ritual_definition("wasm_test", None);
        let ritual = Ritual::new(definition);
        let mut state = create_test_state();
        let mut result = RitualResult {
            ritual_name: "test".to_string(),
            execution_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            duration_ms: 0,
            symbolic_outputs: HashMap::new(),
            state_changes: Vec::new(),
            emergent_symbols: Vec::new(),
            completion_status: CompletionStatus::Complete,
            resonance_level: 0.0,
        };

        // Test different WASM result codes
        ritual.process_wasm_result(0, &mut state, &mut result);
        assert!(result.emergent_symbols.contains(&"✓".to_string()));

        ritual.process_wasm_result(1, &mut state, &mut result);
        assert!(result.emergent_symbols.contains(&"⚡".to_string()));

        ritual.process_wasm_result(2, &mut state, &mut result);
        assert!(result.emergent_symbols.contains(&"🔄".to_string()));

        ritual.process_wasm_result(99, &mut state, &mut result);
        assert!(result.emergent_symbols.contains(&"?".to_string()));
    }
}
