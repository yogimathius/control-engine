use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents an archetypal force within the psyche
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archetype {
    pub id: Uuid,
    pub name: String,
    pub essence: String,
    pub activation_level: f64, // 0.0 to 1.0
    pub shadow_aspects: Vec<String>,
    pub light_aspects: Vec<String>,
    pub last_invoked: Option<DateTime<Utc>>,
    pub evolution_count: u32,
}

impl Archetype {
    pub fn new(name: String, essence: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            essence,
            activation_level: 0.0,
            shadow_aspects: Vec::new(),
            light_aspects: Vec::new(),
            last_invoked: None,
            evolution_count: 0,
        }
    }

    pub fn invoke(&mut self, intensity: f64) {
        self.activation_level = (self.activation_level + intensity).min(1.0);
        self.last_invoked = Some(Utc::now());
        self.evolution_count += 1;
    }

    pub fn integrate_aspect(&mut self, aspect: String, is_shadow: bool) {
        if is_shadow {
            self.shadow_aspects.push(aspect);
        } else {
            self.light_aspects.push(aspect);
        }
    }
}

/// Represents energetic states and flows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Energy {
    pub id: Uuid,
    pub name: String,
    pub frequency: f64,
    pub amplitude: f64,
    pub polarity: Polarity,
    pub elemental_association: Element,
    pub last_shifted: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Polarity {
    Positive,
    Negative,
    Neutral,
    Oscillating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Earth,
    Air,
    Void,
    Light,
    Shadow,
}

impl Energy {
    pub fn new(name: String, frequency: f64, element: Element) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            frequency,
            amplitude: 0.5,
            polarity: Polarity::Neutral,
            elemental_association: element,
            last_shifted: Utc::now(),
        }
    }

    pub fn modulate(&mut self, frequency_shift: f64, amplitude_shift: f64) {
        self.frequency += frequency_shift;
        self.amplitude = (self.amplitude + amplitude_shift).clamp(0.0, 1.0);
        self.last_shifted = Utc::now();
    }
}

/// Represents integrated wisdom and realizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    pub id: Uuid,
    pub name: String,
    pub wisdom: String,
    pub archetypes_involved: Vec<Uuid>,
    pub integration_date: DateTime<Utc>,
    pub depth_level: u8, // 1-10 scale
    pub embodiment_status: EmbodimentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbodimentStatus {
    Conceptual,
    Emotional,
    Energetic,
    Physical,
    Transcendent,
}

impl Integration {
    pub fn new(name: String, wisdom: String, archetypes: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            wisdom,
            archetypes_involved: archetypes,
            integration_date: Utc::now(),
            depth_level: 1,
            embodiment_status: EmbodimentStatus::Conceptual,
        }
    }

    pub fn deepen(&mut self, levels: u8) {
        self.depth_level = (self.depth_level + levels).min(10);
    }
}

/// The complete symbolic state of the being
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicState {
    pub archetypes: HashMap<String, Archetype>,
    pub energies: HashMap<String, Energy>,
    pub integrations: HashMap<String, Integration>,
    pub unresolved_symbols: Vec<String>,
    pub active_transformations: Vec<String>,
    pub last_updated: DateTime<Utc>,
    pub evolution_cycle: u32,
}

impl Default for SymbolicState {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolicState {
    pub fn new() -> Self {
        Self {
            archetypes: HashMap::new(),
            energies: HashMap::new(),
            integrations: HashMap::new(),
            unresolved_symbols: Vec::new(),
            active_transformations: Vec::new(),
            last_updated: Utc::now(),
            evolution_cycle: 0,
        }
    }

    pub fn add_archetype(&mut self, archetype: Archetype) {
        let name = archetype.name.clone();
        self.archetypes.insert(name, archetype);
        self.mark_updated();
    }

    pub fn add_energy(&mut self, energy: Energy) {
        let name = energy.name.clone();
        self.energies.insert(name, energy);
        self.mark_updated();
    }

    pub fn add_integration(&mut self, integration: Integration) {
        let name = integration.name.clone();
        self.integrations.insert(name, integration);
        self.mark_updated();
    }

    pub fn add_unresolved_symbol(&mut self, symbol: String) {
        self.unresolved_symbols.push(symbol);
        self.mark_updated();
    }

    pub fn resolve_symbol(&mut self, symbol: &str) -> bool {
        if let Some(pos) = self.unresolved_symbols.iter().position(|s| s == symbol) {
            self.unresolved_symbols.remove(pos);
            self.mark_updated();
            true
        } else {
            false
        }
    }

    pub fn begin_transformation(&mut self, transformation: String) {
        self.active_transformations.push(transformation);
        self.mark_updated();
    }

    pub fn complete_transformation(&mut self, transformation: &str) -> bool {
        if let Some(pos) = self
            .active_transformations
            .iter()
            .position(|t| t == transformation)
        {
            self.active_transformations.remove(pos);
            self.evolution_cycle += 1;
            self.mark_updated();
            true
        } else {
            false
        }
    }

    fn mark_updated(&mut self) {
        self.last_updated = Utc::now();
    }

    pub fn get_activation_summary(&self) -> String {
        let total_archetypes = self.archetypes.len();
        let active_archetypes = self
            .archetypes
            .values()
            .filter(|a| a.activation_level > 0.1)
            .count();

        let total_energy = self.energies.values().map(|e| e.amplitude).sum::<f64>();

        format!(
            "Archetypes: {}/{} active | Energy: {:.2} | Integrations: {} | Transformations: {}",
            active_archetypes,
            total_archetypes,
            total_energy,
            self.integrations.len(),
            self.active_transformations.len()
        )
    }
}

/// Simplified state structure for web API compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypalState {
    pub archetypes: HashMap<String, f64>,
    pub energies: HashMap<String, f64>,
    pub integrations: Vec<String>,
    pub symbols: Vec<String>,
    pub transformations: Vec<String>,
}

impl ArchetypalState {
    pub fn new() -> Self {
        let mut state = Self {
            archetypes: HashMap::new(),
            energies: HashMap::new(),
            integrations: Vec::new(),
            symbols: Vec::new(),
            transformations: Vec::new(),
        };

        // Initialize with default archetypes and energies
        state.archetypes.insert("Sage".to_string(), 0.1);
        state.archetypes.insert("Shadow".to_string(), 0.1);
        state.archetypes.insert("Anima".to_string(), 0.1);
        state.archetypes.insert("Creator".to_string(), 0.1);

        state.energies.insert("Fire".to_string(), 0.3);
        state.energies.insert("Water".to_string(), 0.3);
        state.energies.insert("Earth".to_string(), 0.3);
        state.energies.insert("Air".to_string(), 0.3);
        state.energies.insert("Void".to_string(), 0.2);

        state
    }

    /// Convert from the full SymbolicState to simplified ArchetypalState
    pub fn from_symbolic_state(symbolic: &SymbolicState) -> Self {
        let archetypes = symbolic
            .archetypes
            .iter()
            .map(|(name, archetype)| (name.clone(), archetype.activation_level))
            .collect();

        let energies = symbolic
            .energies
            .iter()
            .map(|(name, energy)| (name.clone(), energy.amplitude))
            .collect();

        let integrations = symbolic.integrations.keys().cloned().collect();

        let symbols = symbolic.unresolved_symbols.clone();
        let transformations = symbolic.active_transformations.clone();

        Self {
            archetypes,
            energies,
            integrations,
            symbols,
            transformations,
        }
    }

    /// Convert to full SymbolicState
    pub fn to_symbolic_state(&self) -> SymbolicState {
        let mut symbolic = SymbolicState::new();

        // Convert archetypes
        for (name, &activation) in &self.archetypes {
            let mut archetype =
                Archetype::new(name.clone(), format!("Archetypal force of {}", name));
            archetype.activation_level = activation;
            symbolic.add_archetype(archetype);
        }

        // Convert energies
        for (name, &amplitude) in &self.energies {
            let element = match name.as_str() {
                "Fire" => Element::Fire,
                "Water" => Element::Water,
                "Earth" => Element::Earth,
                "Air" => Element::Air,
                "Void" => Element::Void,
                "Light" => Element::Light,
                "Shadow" => Element::Shadow,
                _ => Element::Void,
            };
            let mut energy = Energy::new(name.clone(), 440.0, element);
            energy.amplitude = amplitude;
            symbolic.add_energy(energy);
        }

        // Add integrations, symbols, and transformations
        for integration_name in &self.integrations {
            let integration = Integration::new(
                integration_name.clone(),
                format!("Integration of {}", integration_name),
                Vec::new(),
            );
            symbolic.add_integration(integration);
        }

        symbolic.unresolved_symbols = self.symbols.clone();
        symbolic.active_transformations = self.transformations.clone();

        symbolic
    }
}

impl Default for ArchetypalState {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a ritual execution session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualSession {
    pub ritual_name: String,
    pub intention: String,
    pub pre_state: ArchetypalState,
    pub post_state: ArchetypalState,
    pub transformation_intensity: f64,
    pub execution_duration: std::time::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_creation() {
        let archetype = Archetype::new(
            "Shadow".to_string(),
            "The dark aspects of the psyche".to_string(),
        );

        assert_eq!(archetype.name, "Shadow");
        assert_eq!(archetype.essence, "The dark aspects of the psyche");
        assert_eq!(archetype.activation_level, 0.0);
        assert!(archetype.shadow_aspects.is_empty());
        assert!(archetype.light_aspects.is_empty());
        assert!(archetype.last_invoked.is_none());
        assert_eq!(archetype.evolution_count, 0);
    }

    #[test]
    fn test_archetype_invoke() {
        let mut archetype = Archetype::new("Sage".to_string(), "Wisdom and knowledge".to_string());

        archetype.invoke(0.5);

        assert_eq!(archetype.activation_level, 0.5);
        assert!(archetype.last_invoked.is_some());
        assert_eq!(archetype.evolution_count, 1);
    }

    #[test]
    fn test_archetype_invoke_max_cap() {
        let mut archetype = Archetype::new("Creator".to_string(), "Creative force".to_string());

        archetype.invoke(0.8);
        archetype.invoke(0.5); // Should cap at 1.0

        assert_eq!(archetype.activation_level, 1.0);
        assert_eq!(archetype.evolution_count, 2);
    }

    #[test]
    fn test_archetype_integrate_shadow_aspect() {
        let mut archetype = Archetype::new("Hero".to_string(), "The heroic journey".to_string());

        archetype.integrate_aspect("Pride".to_string(), true);

        assert_eq!(archetype.shadow_aspects.len(), 1);
        assert_eq!(archetype.shadow_aspects[0], "Pride");
        assert!(archetype.light_aspects.is_empty());
    }

    #[test]
    fn test_archetype_integrate_light_aspect() {
        let mut archetype = Archetype::new("Healer".to_string(), "The healing force".to_string());

        archetype.integrate_aspect("Compassion".to_string(), false);

        assert_eq!(archetype.light_aspects.len(), 1);
        assert_eq!(archetype.light_aspects[0], "Compassion");
        assert!(archetype.shadow_aspects.is_empty());
    }

    #[test]
    fn test_energy_creation() {
        let energy = Energy::new("Fire".to_string(), 528.0, Element::Fire);

        assert_eq!(energy.name, "Fire");
        assert_eq!(energy.frequency, 528.0);
        assert_eq!(energy.amplitude, 0.5);
        assert!(matches!(energy.polarity, Polarity::Neutral));
        assert!(matches!(energy.elemental_association, Element::Fire));
    }

    #[test]
    fn test_energy_modulate() {
        let mut energy = Energy::new("Water".to_string(), 396.0, Element::Water);
        let initial_time = energy.last_shifted;

        std::thread::sleep(std::time::Duration::from_millis(1));
        energy.modulate(100.0, -0.2);

        assert_eq!(energy.frequency, 496.0);
        assert_eq!(energy.amplitude, 0.3);
        assert!(energy.last_shifted > initial_time);
    }

    #[test]
    fn test_energy_amplitude_clamp() {
        let mut energy = Energy::new("Shadow".to_string(), 100.0, Element::Shadow);

        energy.modulate(0.0, -1.0); // Should clamp to 0.0
        assert_eq!(energy.amplitude, 0.0);

        energy.modulate(0.0, 2.0); // Should clamp to 1.0
        assert_eq!(energy.amplitude, 1.0);
    }

    #[test]
    fn test_integration_creation() {
        let archetype_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
        let integration = Integration::new(
            "Shadow Work".to_string(),
            "Embracing the dark aspects leads to wholeness".to_string(),
            archetype_ids.clone(),
        );

        assert_eq!(integration.name, "Shadow Work");
        assert_eq!(
            integration.wisdom,
            "Embracing the dark aspects leads to wholeness"
        );
        assert_eq!(integration.archetypes_involved, archetype_ids);
        assert_eq!(integration.depth_level, 1);
        assert!(matches!(
            integration.embodiment_status,
            EmbodimentStatus::Conceptual
        ));
    }

    #[test]
    fn test_integration_deepen() {
        let mut integration = Integration::new(
            "Self Acceptance".to_string(),
            "Accepting all parts of oneself".to_string(),
            vec![],
        );

        integration.deepen(3);
        assert_eq!(integration.depth_level, 4);

        integration.deepen(10); // Should cap at 10
        assert_eq!(integration.depth_level, 10);
    }

    #[test]
    fn test_symbolic_state_new() {
        let state = SymbolicState::new();

        assert!(state.archetypes.is_empty());
        assert!(state.energies.is_empty());
        assert!(state.integrations.is_empty());
        assert!(state.unresolved_symbols.is_empty());
        assert!(state.active_transformations.is_empty());
        assert_eq!(state.evolution_cycle, 0);
    }

    #[test]
    fn test_symbolic_state_add_archetype() {
        let mut state = SymbolicState::new();
        let archetype = Archetype::new("Warrior".to_string(), "Strength and courage".to_string());
        let archetype_name = archetype.name.clone();

        state.add_archetype(archetype);

        assert_eq!(state.archetypes.len(), 1);
        assert!(state.archetypes.contains_key(&archetype_name));
    }

    #[test]
    fn test_symbolic_state_add_energy() {
        let mut state = SymbolicState::new();
        let energy = Energy::new("Earth".to_string(), 194.18, Element::Earth);
        let energy_name = energy.name.clone();

        state.add_energy(energy);

        assert_eq!(state.energies.len(), 1);
        assert!(state.energies.contains_key(&energy_name));
    }

    #[test]
    fn test_symbolic_state_add_integration() {
        let mut state = SymbolicState::new();
        let integration = Integration::new(
            "Balance".to_string(),
            "Finding harmony between opposites".to_string(),
            vec![],
        );
        let integration_name = integration.name.clone();

        state.add_integration(integration);

        assert_eq!(state.integrations.len(), 1);
        assert!(state.integrations.contains_key(&integration_name));
    }

    #[test]
    fn test_symbolic_state_symbol_management() {
        let mut state = SymbolicState::new();

        state.add_unresolved_symbol("∞".to_string());
        assert_eq!(state.unresolved_symbols.len(), 1);
        assert_eq!(state.unresolved_symbols[0], "∞");

        let resolved = state.resolve_symbol("∞");
        assert!(resolved);
        assert!(state.unresolved_symbols.is_empty());

        let not_resolved = state.resolve_symbol("∅");
        assert!(!not_resolved);
    }

    #[test]
    fn test_symbolic_state_transformation_management() {
        let mut state = SymbolicState::new();
        let initial_cycle = state.evolution_cycle;

        state.begin_transformation("shadow_work".to_string());
        assert_eq!(state.active_transformations.len(), 1);
        assert_eq!(state.active_transformations[0], "shadow_work");
        assert_eq!(state.evolution_cycle, initial_cycle);

        let completed = state.complete_transformation("shadow_work");
        assert!(completed);
        assert!(state.active_transformations.is_empty());
        assert_eq!(state.evolution_cycle, initial_cycle + 1);

        let not_completed = state.complete_transformation("non_existent");
        assert!(!not_completed);
    }

    #[test]
    fn test_archetypal_state_new() {
        let state = ArchetypalState::new();

        assert_eq!(state.archetypes.len(), 4);
        assert_eq!(state.energies.len(), 5);
        assert!(state.integrations.is_empty());
        assert!(state.symbols.is_empty());
        assert!(state.transformations.is_empty());

        // Verify default archetype values
        assert_eq!(state.archetypes.get("Sage"), Some(&0.1));
        assert_eq!(state.archetypes.get("Shadow"), Some(&0.1));
        assert_eq!(state.archetypes.get("Anima"), Some(&0.1));
        assert_eq!(state.archetypes.get("Creator"), Some(&0.1));

        // Verify default energy values
        assert_eq!(state.energies.get("Fire"), Some(&0.3));
        assert_eq!(state.energies.get("Water"), Some(&0.3));
        assert_eq!(state.energies.get("Earth"), Some(&0.3));
        assert_eq!(state.energies.get("Air"), Some(&0.3));
        assert_eq!(state.energies.get("Void"), Some(&0.2));
    }

    #[test]
    fn test_archetypal_state_conversion() {
        let mut symbolic = SymbolicState::new();

        // Add test data
        let mut archetype =
            Archetype::new("Magician".to_string(), "Transformation power".to_string());
        archetype.activation_level = 0.8;
        symbolic.add_archetype(archetype);

        let mut energy = Energy::new("Light".to_string(), 852.0, Element::Light);
        energy.amplitude = 0.9;
        symbolic.add_energy(energy);

        symbolic.add_integration(Integration::new(
            "Inner Wisdom".to_string(),
            "Accessing deep knowing".to_string(),
            vec![],
        ));

        symbolic.add_unresolved_symbol("☽".to_string());
        symbolic.begin_transformation("awakening".to_string());

        // Convert to ArchetypalState
        let archetypal = ArchetypalState::from_symbolic_state(&symbolic);

        assert_eq!(archetypal.archetypes.get("Magician"), Some(&0.8));
        assert_eq!(archetypal.energies.get("Light"), Some(&0.9));
        assert!(archetypal
            .integrations
            .contains(&"Inner Wisdom".to_string()));
        assert!(archetypal.symbols.contains(&"☽".to_string()));
        assert!(archetypal
            .transformations
            .contains(&"awakening".to_string()));

        // Convert back to SymbolicState
        let converted_back = archetypal.to_symbolic_state();

        assert!(converted_back.archetypes.contains_key("Magician"));
        assert!(converted_back.energies.contains_key("Light"));
        assert!(converted_back.integrations.contains_key("Inner Wisdom"));
        assert!(converted_back.unresolved_symbols.contains(&"☽".to_string()));
        assert!(converted_back
            .active_transformations
            .contains(&"awakening".to_string()));
    }

    #[test]
    fn test_get_activation_summary() {
        let mut state = SymbolicState::new();

        // Add archetypes with different activation levels
        let mut active_archetype = Archetype::new("Sage".to_string(), "Wisdom".to_string());
        active_archetype.activation_level = 0.5; // Above 0.1 threshold
        state.add_archetype(active_archetype);

        let mut inactive_archetype = Archetype::new("Fool".to_string(), "Innocence".to_string());
        inactive_archetype.activation_level = 0.05; // Below 0.1 threshold
        state.add_archetype(inactive_archetype);

        // Add energies
        let mut energy1 = Energy::new("Fire".to_string(), 528.0, Element::Fire);
        energy1.amplitude = 0.8;
        state.add_energy(energy1);

        let mut energy2 = Energy::new("Water".to_string(), 396.0, Element::Water);
        energy2.amplitude = 0.6;
        state.add_energy(energy2);

        // Add integration
        state.add_integration(Integration::new(
            "Balance".to_string(),
            "Inner balance".to_string(),
            vec![],
        ));

        // Add transformation
        state.begin_transformation("healing".to_string());

        let summary = state.get_activation_summary();

        assert!(summary.contains("Archetypes: 1/2 active"));
        assert!(summary.contains("Energy: 1.40"));
        assert!(summary.contains("Integrations: 1"));
        assert!(summary.contains("Transformations: 1"));
    }
}
