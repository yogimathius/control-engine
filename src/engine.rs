use crate::{
    Archetype, CodexError, Element, Energy, ReflectionResult, Reflector, Ritual, RitualDefinition,
    RitualResult, SymbolicState,
};
use dirs;
use std::collections::HashMap;
use std::path::PathBuf;

/// The main engine that orchestrates the Codex Control system
pub struct CodexEngine {
    state: SymbolicState,
    rituals: HashMap<String, RitualDefinition>,
    reflector: Reflector,
    data_dir: PathBuf,
    last_ritual_result: Option<RitualResult>,
}

impl CodexEngine {
    pub fn new() -> Result<Self, CodexError> {
        let data_dir = Self::get_data_directory()?;
        std::fs::create_dir_all(&data_dir)?;

        let mut engine = Self {
            state: SymbolicState::new(),
            rituals: HashMap::new(),
            reflector: Reflector::new_with_defaults(),
            data_dir,
            last_ritual_result: None,
        };

        // Load existing state if it exists
        engine.load_state()?;

        // Initialize with foundational rituals
        engine.register_foundational_rituals();

        Ok(engine)
    }

    fn get_data_directory() -> Result<PathBuf, CodexError> {
        let home_dir = dirs::home_dir().ok_or_else(|| CodexError::StateCorruption {
            reason: "Could not find home directory".to_string(),
        })?;

        Ok(home_dir.join(".codex"))
    }

    pub fn load_state(&mut self) -> Result<(), CodexError> {
        let state_file = self.data_dir.join("state.json");

        if state_file.exists() {
            let content = std::fs::read_to_string(&state_file)?;
            self.state = serde_json::from_str(&content)?;
            println!("🔮 Symbolic state loaded from previous session");
        } else {
            // Initialize with primordial archetypes
            self.initialize_primordial_state();
            println!("🌟 Primordial state initialized");
        }

        Ok(())
    }

    pub fn save_state(&self) -> Result<(), CodexError> {
        let state_file = self.data_dir.join("state.json");
        let content = serde_json::to_string_pretty(&self.state)?;
        std::fs::write(&state_file, content)?;
        Ok(())
    }

    fn initialize_primordial_state(&mut self) {
        // Add foundational archetypes
        let sage = Archetype::new(
            "Sage".to_string(),
            "The keeper of wisdom and inner knowing".to_string(),
        );
        self.state.add_archetype(sage);

        let shadow = Archetype::new(
            "Shadow".to_string(),
            "The rejected aspects seeking integration".to_string(),
        );
        self.state.add_archetype(shadow);

        let anima = Archetype::new(
            "Anima".to_string(),
            "The inner feminine wisdom and intuition".to_string(),
        );
        self.state.add_archetype(anima);

        let creator = Archetype::new(
            "Creator".to_string(),
            "The force of manifestation and creativity".to_string(),
        );
        self.state.add_archetype(creator);

        // Add foundational energies
        let earth_energy = Energy::new(
            "Earth".to_string(),
            3.5, // Grounding frequency
            Element::Earth,
        );
        self.state.add_energy(earth_energy);

        let fire_energy = Energy::new(
            "Fire".to_string(),
            9.2, // Transformative frequency
            Element::Fire,
        );
        self.state.add_energy(fire_energy);

        let void_energy = Energy::new(
            "Void".to_string(),
            0.1, // Stillness frequency
            Element::Void,
        );
        self.state.add_energy(void_energy);
    }

    fn register_foundational_rituals(&mut self) {
        // Shadow Integration Ritual
        let shadow_ritual = RitualDefinition {
            name: "shadow_integration".to_string(),
            description: "A ritual to integrate rejected aspects of the self".to_string(),
            intent: "To embrace and transform shadow elements into conscious wisdom".to_string(),
            required_archetypes: vec!["Shadow".to_string(), "Sage".to_string()],
            energy_requirements: HashMap::from([
                ("Fire".to_string(), 0.6),
                ("Void".to_string(), 0.3),
            ]),
            wasm_module_path: None,
            native_handler: Some("shadow_integration".to_string()),
            parameters: HashMap::new(),
        };
        self.rituals
            .insert("shadow_integration".to_string(), shadow_ritual);

        // Energy Attunement Ritual
        let attunement_ritual = RitualDefinition {
            name: "energy_attunement".to_string(),
            description: "A ritual to harmonize energetic frequencies".to_string(),
            intent: "To align personal energy with natural harmonic resonance".to_string(),
            required_archetypes: vec!["Sage".to_string()],
            energy_requirements: HashMap::from([("Earth".to_string(), 0.4)]),
            wasm_module_path: None,
            native_handler: Some("energy_attunement".to_string()),
            parameters: HashMap::new(),
        };
        self.rituals
            .insert("energy_attunement".to_string(), attunement_ritual);

        // Archetype Invocation Ritual
        let invocation_ritual = RitualDefinition {
            name: "archetype_invocation".to_string(),
            description: "A ritual to invoke and activate archetypal energies".to_string(),
            intent: "To awaken dormant archetypal forces within consciousness".to_string(),
            required_archetypes: vec!["Creator".to_string(), "Anima".to_string()],
            energy_requirements: HashMap::from([("Fire".to_string(), 0.7)]),
            wasm_module_path: None,
            native_handler: Some("archetype_invocation".to_string()),
            parameters: HashMap::new(),
        };
        self.rituals
            .insert("archetype_invocation".to_string(), invocation_ritual);

        // Void Contemplation Ritual
        let void_ritual = RitualDefinition {
            name: "void_contemplation".to_string(),
            description: "A ritual of emptiness and infinite potential".to_string(),
            intent: "To enter the void space where all transformation becomes possible".to_string(),
            required_archetypes: vec!["Sage".to_string()],
            energy_requirements: HashMap::from([("Void".to_string(), 0.8)]),
            wasm_module_path: None,
            native_handler: Some("void_contemplation".to_string()),
            parameters: HashMap::new(),
        };
        self.rituals
            .insert("void_contemplation".to_string(), void_ritual);
    }

    pub async fn execute_ritual(&mut self, ritual_name: &str) -> Result<RitualResult, CodexError> {
        let ritual_def = self
            .rituals
            .get(ritual_name)
            .ok_or_else(|| CodexError::RitualNotFound {
                name: ritual_name.to_string(),
            })?
            .clone();

        println!("🔥 Invoking ritual: {}", ritual_name);
        println!("💫 Intent: {}", ritual_def.intent);

        let mut ritual = Ritual::new(ritual_def);

        // Load WASM module if specified
        if ritual.definition.wasm_module_path.is_some() {
            ritual.load_wasm_module()?;
        }

        let result = ritual.execute(&mut self.state).await?;

        // Save the result for potential reflection
        self.last_ritual_result = Some(result.clone());

        // Auto-save state after ritual execution
        self.save_state()?;

        println!(
            "✨ Ritual completed with resonance: {:.3}",
            result.resonance_level
        );
        self.display_ritual_result(&result);

        Ok(result)
    }

    pub async fn reflect(&self) -> Result<ReflectionResult, CodexError> {
        if let Some(last_result) = &self.last_ritual_result {
            println!("🔮 Seeking reflection on the recent ritual...");
            let reflection = self
                .reflector
                .reflect_on_ritual(last_result, &self.state)
                .await?;

            // Display the reflection
            println!("{}", self.reflector.format_reflection_output(&reflection));

            Ok(reflection)
        } else {
            Err(CodexError::StateCorruption {
                reason: "No ritual has been performed to reflect upon".to_string(),
            })
        }
    }

    pub fn view_state(&self) {
        use colored::*;

        println!("\n{}", "═".repeat(70).bright_purple());
        println!("{}", "🌟 CURRENT SYMBOLIC STATE".bright_cyan().bold());
        println!("{}", "═".repeat(70).bright_purple());

        println!("\n{}", "📊 OVERVIEW".bright_yellow().bold());
        println!("  {}", self.state.get_activation_summary().white());
        println!(
            "  Evolution Cycle: {}",
            self.state.evolution_cycle.to_string().bright_green()
        );

        // Display active archetypes
        let active_archetypes: Vec<_> = self
            .state
            .archetypes
            .values()
            .filter(|a| a.activation_level > 0.0)
            .collect();

        if !active_archetypes.is_empty() {
            println!("\n{}", "🏛️  ARCHETYPAL FORCES".bright_yellow().bold());
            for archetype in active_archetypes {
                let activation_bar = self.create_bar(archetype.activation_level, 20);
                println!(
                    "  {} {:.3} {}",
                    archetype.name.bright_white().bold(),
                    archetype.activation_level,
                    activation_bar
                );
                if archetype.evolution_count > 0 {
                    println!(
                        "    {} Evolved {} times",
                        "↗".bright_blue(),
                        archetype.evolution_count.to_string().bright_green()
                    );
                }
            }
        }

        // Display energies
        if !self.state.energies.is_empty() {
            println!("\n{}", "⚡ ENERGETIC FLOWS".bright_magenta().bold());
            for energy in self.state.energies.values() {
                let amplitude_bar = self.create_bar(energy.amplitude, 15);
                println!(
                    "  {} f:{:.2} a:{:.3} {}",
                    energy.name.bright_white().bold(),
                    energy.frequency,
                    energy.amplitude,
                    amplitude_bar
                );
            }
        }

        // Display integrations
        if !self.state.integrations.is_empty() {
            println!("\n{}", "🌀 INTEGRATIONS".bright_green().bold());
            for integration in self.state.integrations.values() {
                println!(
                    "  {} (Depth: {}/10)",
                    integration.name.bright_white().bold(),
                    integration.depth_level.to_string().bright_cyan()
                );
                println!("    {}", integration.wisdom.white());
            }
        }

        // Display unresolved symbols
        if !self.state.unresolved_symbols.is_empty() {
            println!("\n{}", "🔍 UNRESOLVED SYMBOLS".bright_red().bold());
            for symbol in &self.state.unresolved_symbols {
                println!("  {}", symbol.bright_yellow());
            }
        }

        // Display active transformations
        if !self.state.active_transformations.is_empty() {
            println!("\n{}", "🔄 ACTIVE TRANSFORMATIONS".bright_blue().bold());
            for transformation in &self.state.active_transformations {
                println!("  {}", transformation.bright_cyan());
            }
        }

        println!("\n{}", "═".repeat(70).bright_purple());
    }

    pub fn list_available_rituals(&self) {
        use colored::*;

        println!("\n{}", "═".repeat(60).bright_purple());
        println!("{}", "📜 AVAILABLE RITUALS".bright_cyan().bold());
        println!("{}", "═".repeat(60).bright_purple());

        for (name, ritual) in &self.rituals {
            println!("\n{}", name.bright_yellow().bold());
            println!("  {}", ritual.description.white());
            println!("  {}", format!("Intent: {}", ritual.intent).bright_green());

            if !ritual.required_archetypes.is_empty() {
                println!(
                    "  Required archetypes: {}",
                    ritual.required_archetypes.join(", ").bright_magenta()
                );
            }
        }

        println!("\n{}", "═".repeat(60).bright_purple());
    }

    fn display_ritual_result(&self, result: &RitualResult) {
        use colored::*;

        println!("\n{}", "━".repeat(50).bright_blue());
        println!("{}", "⚡ RITUAL OUTCOME".bright_cyan().bold());
        println!("{}", "━".repeat(50).bright_blue());

        println!(
            "Status: {}",
            format!("{:?}", result.completion_status).bright_green()
        );
        println!(
            "Duration: {}ms",
            result.duration_ms.to_string().bright_yellow()
        );
        println!(
            "Resonance: {:.3}",
            result.resonance_level.to_string().bright_magenta()
        );

        if !result.emergent_symbols.is_empty() {
            println!("\nEmergent Symbols:");
            for symbol in &result.emergent_symbols {
                println!("  {}", symbol.bright_yellow());
            }
        }

        if !result.state_changes.is_empty() {
            println!("\nState Changes:");
            for change in &result.state_changes {
                println!(
                    "  {} {}",
                    format!("{:?}:", change.change_type).bright_blue(),
                    change.description.white()
                );
            }
        }

        println!("{}", "━".repeat(50).bright_blue());
    }

    fn create_bar(&self, value: f64, length: usize) -> String {
        use colored::*;

        let filled_length = (value * length as f64) as usize;
        let filled = "█".repeat(filled_length.min(length));
        let empty = "░".repeat(length.saturating_sub(filled_length));

        format!("{}{}", filled.bright_cyan(), empty.dimmed())
    }

    pub fn add_custom_ritual(&mut self, ritual: RitualDefinition) {
        let name = ritual.name.clone();
        self.rituals.insert(name, ritual);
    }

    pub fn get_state(&self) -> &SymbolicState {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut SymbolicState {
        &mut self.state
    }
}
