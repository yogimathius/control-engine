use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    // Host functions that the Codex engine will provide
    #[wasm_bindgen(js_name = "codex_get_archetype_activation")]
    fn get_archetype_activation(name: &str) -> f64;
    
    #[wasm_bindgen(js_name = "codex_set_archetype_activation")]
    fn set_archetype_activation(name: &str, level: f64);
    
    #[wasm_bindgen(js_name = "codex_get_energy_amplitude")]
    fn get_energy_amplitude(name: &str) -> f64;
    
    #[wasm_bindgen(js_name = "codex_set_energy_amplitude")]
    fn set_energy_amplitude(name: &str, amplitude: f64);
    
    #[wasm_bindgen(js_name = "codex_add_symbol")]
    fn add_symbol(symbol: &str);
    
    #[wasm_bindgen(js_name = "codex_random")]
    fn random() -> f64;
}

// Define a macro for easier logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct RitualResult {
    pub success: bool,
    pub resonance_level: f64,
    pub symbols_added: Vec<String>,
    pub message: String,
}

/// Shadow Integration Ritual - Rust/WASM Implementation
#[wasm_bindgen]
pub fn shadow_integration_ritual() -> JsValue {
    console_error_panic_hook::set_once();
    console_log!("🔮 Starting Shadow Integration Ritual");
    
    // Get current shadow and light levels
    let shadow_level = get_archetype_activation("Shadow");
    let light_level = get_archetype_activation("Light");
    
    console_log!("Current Shadow: {:.2}, Light: {:.2}", shadow_level, light_level);
    
    // Calculate integration factor based on imbalance and randomness
    let imbalance = (shadow_level - light_level).abs();
    let base_factor = 0.2 + (random() * 0.3);
    let integration_factor = base_factor * (1.0 + imbalance);
    
    // Increase shadow activation (conscious integration)
    let new_shadow = (shadow_level + integration_factor).min(1.0);
    set_archetype_activation("Shadow", new_shadow);
    
    // Slight increase in light for balance
    let new_light = (light_level + integration_factor * 0.3).min(1.0);
    set_archetype_activation("Light", new_light);
    
    // Add integration symbols
    let mut symbols_added = Vec::new();
    add_symbol("◯●◯");
    symbols_added.push("◯●◯".to_string());
    
    add_symbol("🌑");
    symbols_added.push("🌑".to_string());
    
    // Add energy symbol for high integration
    if integration_factor > 0.4 {
        add_symbol("⚡");
        symbols_added.push("⚡".to_string());
    }
    
    // Calculate final resonance
    let final_shadow = get_archetype_activation("Shadow");
    let final_light = get_archetype_activation("Light");
    let balance = 1.0 - (final_shadow - final_light).abs();
    let resonance = ((final_shadow + balance) * 0.5).min(1.0);
    
    console_log!("🌟 Shadow Integration Complete - Resonance: {:.2}", resonance);
    
    let result = RitualResult {
        success: true,
        resonance_level: resonance,
        symbols_added,
        message: "Shadow aspects integrated. Wholeness approaches.".to_string(),
    };
    
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Energy Attunement Ritual - Balances elemental energies
#[wasm_bindgen]
pub fn energy_attunement_ritual() -> JsValue {
    console_error_panic_hook::set_once();
    console_log!("🔮 Starting Energy Attunement Ritual");
    
    // Get current energy levels
    let fire_amp = get_energy_amplitude("Fire");
    let water_amp = get_energy_amplitude("Water");
    let earth_amp = get_energy_amplitude("Earth");
    let air_amp = get_energy_amplitude("Air");
    
    console_log!("Energies - Fire: {:.2}, Water: {:.2}, Earth: {:.2}, Air: {:.2}", 
                fire_amp, water_amp, earth_amp, air_amp);
    
    // Calculate balanced target level
    let total_energy = fire_amp + water_amp + earth_amp + air_amp;
    let target_level = (total_energy / 4.0) + (random() * 0.1);
    let adjustment = 0.3; // Gentle adjustment factor
    
    // Adjust each energy toward balance
    let new_fire = fire_amp + (target_level - fire_amp) * adjustment;
    let new_water = water_amp + (target_level - water_amp) * adjustment;
    let new_earth = earth_amp + (target_level - earth_amp) * adjustment;
    let new_air = air_amp + (target_level - air_amp) * adjustment;
    
    set_energy_amplitude("Fire", new_fire);
    set_energy_amplitude("Water", new_water);
    set_energy_amplitude("Earth", new_earth);
    set_energy_amplitude("Air", new_air);
    
    // Add harmonic symbols
    let mut symbols_added = Vec::new();
    add_symbol("∿∿∿");
    symbols_added.push("∿∿∿".to_string());
    
    add_symbol("⚡");
    symbols_added.push("⚡".to_string());
    
    // Add elemental symbols for high total energy
    if total_energy > 2.0 {
        add_symbol("🔥");
        add_symbol("💧");
        symbols_added.push("🔥".to_string());
        symbols_added.push("💧".to_string());
    }
    
    // Calculate resonance based on balance
    let variance = (new_fire - target_level).abs() + (new_water - target_level).abs() +
                   (new_earth - target_level).abs() + (new_air - target_level).abs();
    let resonance = (1.0 - variance.min(0.8)).max(0.2);
    
    console_log!("🌟 Energy Attunement Complete - Resonance: {:.2}", resonance);
    
    let result = RitualResult {
        success: true,
        resonance_level: resonance,
        symbols_added,
        message: "Elemental energies harmonized. Balance restored.".to_string(),
    };
    
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Void Contemplation Ritual - Deep meditation practice
#[wasm_bindgen]
pub fn void_contemplation_ritual() -> JsValue {
    console_error_panic_hook::set_once();
    console_log!("🔮 Starting Void Contemplation Ritual");
    
    // Increase Void energy significantly
    let void_current = get_energy_amplitude("Void");
    let void_increase = 0.3 + (random() * 0.2);
    let new_void = (void_current + void_increase).min(1.0);
    set_energy_amplitude("Void", new_void);
    
    // Activate Sage and Mystic archetypes
    let sage_level = get_archetype_activation("Sage");
    let mystic_level = get_archetype_activation("Mystic");
    
    set_archetype_activation("Sage", (sage_level + 0.15).min(1.0));
    set_archetype_activation("Mystic", (mystic_level + 0.2).min(1.0));
    
    // Add contemplative symbols
    let mut symbols_added = Vec::new();
    add_symbol("○");
    symbols_added.push("○".to_string());
    
    add_symbol("∞");
    symbols_added.push("∞".to_string());
    
    // High void energy adds deeper symbols
    if new_void > 0.7 {
        add_symbol("◯");
        add_symbol("⚬");
        symbols_added.push("◯".to_string());
        symbols_added.push("⚬".to_string());
    }
    
    let resonance = (new_void * 0.8 + random() * 0.2).min(1.0);
    
    console_log!("🌟 Void Contemplation Complete - Resonance: {:.2}", resonance);
    
    let result = RitualResult {
        success: true,
        resonance_level: resonance,
        symbols_added,
        message: "Touched the infinite void. Ego dissolves into being.".to_string(),
    };
    
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Archetype Invocation - General archetype activation
#[wasm_bindgen] 
pub fn archetype_invocation_ritual() -> JsValue {
    console_error_panic_hook::set_once();
    console_log!("🔮 Starting Archetype Invocation Ritual");
    
    // Get all major archetypes and boost them slightly
    let archetypes = ["Sage", "Creator", "Shadow", "Light", "Warrior", "Lover", "Ruler", "Magician"];
    let boost = 0.1 + (random() * 0.1);
    
    for archetype in archetypes.iter() {
        let current = get_archetype_activation(archetype);
        let new_level = (current + boost).min(1.0);
        set_archetype_activation(archetype, new_level);
    }
    
    // Add invocation symbols
    let mut symbols_added = Vec::new();
    add_symbol("🔮");
    symbols_added.push("🔮".to_string());
    
    add_symbol("∆∇∆");
    symbols_added.push("∆∇∆".to_string());
    
    // Calculate resonance based on archetype diversity
    let total_activation: f64 = archetypes.iter()
        .map(|&arch| get_archetype_activation(arch))
        .sum();
    let resonance = (total_activation / archetypes.len() as f64 * 0.9).min(1.0);
    
    console_log!("🌟 Archetype Invocation Complete - Resonance: {:.2}", resonance);
    
    let result = RitualResult {
        success: true,
        resonance_level: resonance,
        symbols_added,
        message: "Archetypal forces awakened. The inner pantheon stirs.".to_string(),
    };
    
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Test function to verify WASM module loading
#[wasm_bindgen]
pub fn test_ritual() -> JsValue {
    console_log!("✨ WASM Ritual Module Loaded Successfully!");
    
    let result = RitualResult {
        success: true,
        resonance_level: 1.0,
        symbols_added: vec!["✨".to_string()],
        message: "WASM ritual system operational".to_string(),
    };
    
    serde_wasm_bindgen::to_value(&result).unwrap()
}