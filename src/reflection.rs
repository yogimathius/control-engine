use crate::{CodexError, RitualResult, SymbolicState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionResult {
    pub ritual_name: String,
    pub timestamp: DateTime<Utc>,
    pub archetypal_interpretation: String,
    pub symbolic_meaning: String,
    pub integration_guidance: String,
    pub emergent_insights: Vec<String>,
    pub resonance_analysis: String,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionConfig {
    pub api_base_url: String,
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for ReflectionConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://openrouter.ai/api/v1".to_string(),
            api_key: std::env::var("OPENROUTER_API_KEY").unwrap_or_default(),
            model: "anthropic/claude-3.5-sonnet".to_string(),
            temperature: 0.7,
            max_tokens: 2000,
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

/// The AI reflection engine
pub struct Reflector {
    config: ReflectionConfig,
    client: reqwest::Client,
}

impl Reflector {
    pub fn new(config: ReflectionConfig) -> Self {
        let client = reqwest::Client::new();
        Self { config, client }
    }

    pub fn new_with_defaults() -> Self {
        Self::new(ReflectionConfig::default())
    }

    // Enhanced reflection methods for better mock responses
    fn generate_archetypal_interpretation(&self, ritual_result: &RitualResult, state: &SymbolicState) -> String {
        match ritual_result.ritual_name.as_str() {
            "shadow_integration" => {
                format!("The shadow integration ritual has activated deep archetypal currents within your psyche. \
                With resonance level {:.2}, you've successfully begun the sacred work of bringing unconscious \
                shadow material into conscious awareness. The {} archetype(s) present in your symbolic state \
                indicate readiness for this profound inner work. This is not mere psychological exercise, but \
                alchemical transformation of the soul.", 
                ritual_result.resonance_level, 
                state.archetypes.len())
            },
            "energy_attunement" => {
                format!("The elemental forces within you have been harmonized through this sacred attunement. \
                Your resonance of {:.2} reflects the cosmic order seeking balance within your energetic being. \
                The {} energy currents now flow in greater harmony, creating coherence between your inner \
                elements and the archetypal forces they serve.",
                ritual_result.resonance_level,
                state.energies.len())
            },
            "void_contemplation" => {
                format!("You have touched the primordial emptiness, the fertile void from which all creation springs. \
                This resonance of {:.2} indicates deep contact with the transcendent realm beyond form. \
                The dissolution of ego boundaries allows pure awareness to emerge, connecting you with \
                the infinite source of all archetypal energy.",
                ritual_result.resonance_level)
            },
            _ => {
                format!("A significant archetypal transformation has occurred through the {} ritual. \
                Your resonance level of {:.2} indicates the depth of inner change achieved. \
                The unconscious has been stirred, and new patterns of meaning are emerging.",
                ritual_result.ritual_name,
                ritual_result.resonance_level)
            }
        }
    }

    fn generate_symbolic_meaning(&self, symbols: &[String]) -> String {
        if symbols.is_empty() {
            return "No new symbols emerged, indicating a period of inner stillness and preparation.".to_string();
        }

        let mut meanings = Vec::new();
        
        for symbol in symbols {
            match symbol.as_str() {
                "◯●◯" => meanings.push("The trinity of shadow integration - conscious, unconscious, and the unified whole"),
                "🌑" => meanings.push("New moon consciousness - the dark fertile void of potential"),
                "⚡" => meanings.push("Energetic activation - the lightning flash of illumination"),
                "∿∿∿" => meanings.push("Harmonic waves - the restoration of natural energetic flow"),
                "🔮" => meanings.push("Archetypal awakening - the activation of primordial wisdom"),
                "○" => meanings.push("The sacred circle - wholeness, completion, and eternal return"),
                "∞" => meanings.push("Infinite consciousness - transcendence of linear limitations"),
                _ => meanings.push("A unique archetypal emergence requiring personal contemplation"),
            }
        }

        format!("The emergent symbols carry profound meaning: {}. These symbols serve as talismans of transformation, \
        anchoring the ritual's effects in your psyche and providing focal points for continued integration work.",
        meanings.join("; "))
    }

    fn generate_integration_guidance(&self, ritual_result: &RitualResult) -> String {
        let base_guidance = match ritual_result.ritual_name.as_str() {
            "shadow_integration" => "Honor the shadow aspects that have surfaced. Create a sacred space for dialogue with these parts of yourself through active imagination or journaling. Practice shadow work gradually - integrate slowly to avoid overwhelming the conscious mind.",
            "energy_attunement" => "Maintain the energetic harmony achieved through consistent practice. Spend time in nature to ground the attunement. Notice how the balanced energies affect your daily interactions and creative expression.",
            "void_contemplation" => "Carry the spaciousness of void consciousness into daily life. Practice moments of emptiness meditation. Allow the ego to rest in the vast awareness you've touched.",
            _ => "Create space for the transformation to settle. Trust the process and allow the changes to integrate naturally over time.",
        };

        let resonance_guidance = if ritual_result.resonance_level > 0.8 {
            " Your high resonance indicates profound change - be gentle with yourself as these new patterns establish."
        } else if ritual_result.resonance_level > 0.6 {
            " Your good resonance suggests solid progress - continue with regular practice to deepen the integration."
        } else {
            " Your resonance indicates gentle progress - be patient with the process and trust in gradual transformation."
        };

        format!("{}{}", base_guidance, resonance_guidance)
    }

    fn generate_emergent_insights(&self, ritual_result: &RitualResult, _state: &SymbolicState) -> Vec<String> {
        let mut insights = Vec::new();

        match ritual_result.ritual_name.as_str() {
            "shadow_integration" => {
                insights.push("What you resist in others often reflects unintegrated aspects of yourself".to_string());
                insights.push("Shadow work is not about eliminating darkness, but making it conscious".to_string());
                insights.push("Integration brings wholeness, not perfection".to_string());
            },
            "energy_attunement" => {
                insights.push("Inner harmony reflects in outer circumstances".to_string());
                insights.push("Balanced energy flows naturally without forcing".to_string());
                insights.push("Elemental awareness connects you to natural wisdom".to_string());
            },
            "void_contemplation" => {
                insights.push("Emptiness is not void of content, but pregnant with infinite possibility".to_string());
                insights.push("The ego dissolves to reveal the eternal Self".to_string());
                insights.push("In stillness, the deepest truths emerge".to_string());
            },
            _ => {
                insights.push("Transformation happens in spirals, not straight lines".to_string());
                insights.push("Trust the wisdom of your unconscious processes".to_string());
            }
        }

        if ritual_result.resonance_level > 0.9 {
            insights.push("Exceptional resonance indicates readiness for advanced practices".to_string());
        }

        insights
    }

    fn suggest_next_steps(&self, ritual_result: &RitualResult) -> Vec<String> {
        let mut steps = Vec::new();

        match ritual_result.ritual_name.as_str() {
            "shadow_integration" => {
                steps.push("Continue shadow work through dream analysis".to_string());
                steps.push("Explore creative expression of integrated aspects".to_string());
                steps.push("Consider archetype invocation to strengthen supportive forces".to_string());
            },
            "energy_attunement" => {
                steps.push("Practice daily elemental meditations".to_string());
                steps.push("Explore void contemplation to deepen energetic awareness".to_string());
                steps.push("Work with natural elements directly (earth, fire, water, air)".to_string());
            },
            "void_contemplation" => {
                steps.push("Integrate void awareness into daily activities".to_string());
                steps.push("Explore archetypal invocation from emptiness".to_string());
                steps.push("Practice action from non-action (wu wei)".to_string());
            },
            _ => {
                steps.push("Allow integration time through rest and reflection".to_string());
                steps.push("Explore complementary practices".to_string());
            }
        }

        if ritual_result.resonance_level < 0.5 {
            steps.push("Focus on foundational practices before advancing".to_string());
        }

        steps
    }

    fn analyze_resonance(&self, ritual_result: &RitualResult) -> String {
        let level = ritual_result.resonance_level;
        
        let quality = if level > 0.9 {
            "Exceptional - profound transformation achieved"
        } else if level > 0.8 {
            "Very High - significant archetypal activation"
        } else if level > 0.7 {
            "High - strong energetic coherence established"
        } else if level > 0.6 {
            "Good - meaningful progress in integration"
        } else if level > 0.5 {
            "Moderate - gentle transformation in progress"
        } else if level > 0.3 {
            "Developing - early stages of change"
        } else {
            "Initial - foundation being established"
        };

        format!("Resonance Level: {:.2} - {}. This indicates the degree of coherence between your \
        conscious intent and the archetypal forces activated. The {} state changes during this ritual \
        demonstrate active transformation occurring within your symbolic matrix.",
        level, quality, ritual_result.state_changes.len())
    }

    pub async fn reflect_on_ritual(
        &self,
        ritual_result: &RitualResult,
        state: &SymbolicState,
    ) -> Result<ReflectionResult, CodexError> {
        // Check if API key is available, fall back to mock if not
        if self.config.api_key.is_empty() {
            tracing::warn!("No API key provided, using enhanced mock reflection");
            return self.create_enhanced_mock_reflection(ritual_result, state);
        }

        let context = self.build_reflection_context(ritual_result, state);
        
        match self.query_ai_oracle(&context, ritual_result).await {
            Ok(ai_response) => self.parse_ai_reflection(ai_response, ritual_result),
            Err(e) => {
                tracing::warn!("AI reflection failed, using enhanced fallback: {}", e);
                self.create_enhanced_mock_reflection(ritual_result, state)
            }
        }
    }

    async fn query_ai_oracle(
        &self,
        context: &str,
        ritual_result: &RitualResult,
    ) -> Result<String, CodexError> {
        let system_prompt = r#"You are a wise archetypal oracle, versed in Jungian psychology, shamanic wisdom, and sacred transformation practices. You interpret symbolic states and transformations with depth, compassion, and practical guidance.

Respond with structured insights in this format:

ARCHETYPAL_INTERPRETATION: [Your interpretation of the archetypal significance]

SYMBOLIC_MEANING: [Analysis of the symbols and their meaning]

INTEGRATION_GUIDANCE: [Practical advice for integrating the transformation]

EMERGENT_INSIGHTS: [List key insights, separated by |]

RESONANCE_ANALYSIS: [Analysis of the energetic resonance and alignment]

NEXT_STEPS: [Recommended next actions, separated by |]"#;

        let user_prompt = format!(
            r#"Sacred Oracle Interpretation Request:

A practitioner has completed the ritual "{}" with resonance level {:.2}.

CONTEXT:
{}

RITUAL OUTCOMES:
- Duration: {}ms
- State Changes: {} transformations
- Emergent Symbols: {}
- Completion Status: {:?}

Please provide your archetypal interpretation and guidance for this sacred transformation."#,
            ritual_result.ritual_name,
            ritual_result.resonance_level,
            context,
            ritual_result.duration_ms,
            ritual_result.state_changes.len(),
            ritual_result.emergent_symbols.join(", "),
            ritual_result.completion_status
        );

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.config.api_base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("HTTP-Referer", "https://codex-control-engine.sacred.dev")
            .json(&request)
            .send()
            .await
            .map_err(|e| CodexError::Network(e))?;

        if !response.status().is_success() {
            return Err(CodexError::ReflectionFailed {
                error: format!("API request failed: {}", response.status()),
            });
        }

        let ai_response: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| CodexError::Network(e))?;

        ai_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| CodexError::ReflectionFailed {
                error: "No response from AI oracle".to_string(),
            })
    }

    fn parse_ai_reflection(
        &self,
        ai_response: String,
        ritual_result: &RitualResult,
    ) -> Result<ReflectionResult, CodexError> {
        let mut reflection = ReflectionResult {
            ritual_name: ritual_result.ritual_name.clone(),
            timestamp: Utc::now(),
            archetypal_interpretation: String::new(),
            symbolic_meaning: String::new(),
            integration_guidance: String::new(),
            emergent_insights: Vec::new(),
            resonance_analysis: String::new(),
            next_steps: Vec::new(),
        };

        // Parse structured response
        for line in ai_response.lines() {
            if let Some(content) = line.strip_prefix("ARCHETYPAL_INTERPRETATION: ") {
                reflection.archetypal_interpretation = content.to_string();
            } else if let Some(content) = line.strip_prefix("SYMBOLIC_MEANING: ") {
                reflection.symbolic_meaning = content.to_string();
            } else if let Some(content) = line.strip_prefix("INTEGRATION_GUIDANCE: ") {
                reflection.integration_guidance = content.to_string();
            } else if let Some(content) = line.strip_prefix("EMERGENT_INSIGHTS: ") {
                reflection.emergent_insights = content
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            } else if let Some(content) = line.strip_prefix("RESONANCE_ANALYSIS: ") {
                reflection.resonance_analysis = content.to_string();
            } else if let Some(content) = line.strip_prefix("NEXT_STEPS: ") {
                reflection.next_steps = content
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }

        // Fallback to default values if parsing failed
        if reflection.archetypal_interpretation.is_empty() {
            reflection.archetypal_interpretation = format!(
                "The ritual '{}' represents a journey of inner transformation and archetypal activation.",
                ritual_result.ritual_name
            );
        }

        if reflection.symbolic_meaning.is_empty() {
            reflection.symbolic_meaning = format!(
                "The symbols {} emerge as markers of unconscious content seeking integration.",
                ritual_result.emergent_symbols.join(", ")
            );
        }

        if reflection.integration_guidance.is_empty() {
            reflection.integration_guidance = "Focus on embodying these insights through mindful practice and conscious reflection.".to_string();
        }

        if reflection.resonance_analysis.is_empty() {
            reflection.resonance_analysis = format!(
                "Resonance level of {:.2} indicates the depth of archetypal engagement achieved.",
                ritual_result.resonance_level
            );
        }

        Ok(reflection)
    }

    fn create_enhanced_mock_reflection(
        &self,
        ritual_result: &RitualResult,
        state: &SymbolicState,
    ) -> Result<ReflectionResult, CodexError> {
        // Enhanced reflection based on actual ritual data and state
        let interpretation = self.generate_archetypal_interpretation(ritual_result, state);
        let meaning = self.generate_symbolic_meaning(&ritual_result.emergent_symbols);
        let guidance = self.generate_integration_guidance(ritual_result);
        let insights = self.generate_emergent_insights(ritual_result, state);

        Ok(ReflectionResult {
            ritual_name: ritual_result.ritual_name.clone(),
            timestamp: chrono::Utc::now(),
            archetypal_interpretation: interpretation,
            symbolic_meaning: meaning,
            integration_guidance: guidance,
            emergent_insights: insights,
            next_steps: self.suggest_next_steps(ritual_result),
            resonance_analysis: self.analyze_resonance(ritual_result),
        })
    }

    fn create_mock_reflection(
        &self,
        ritual_result: &RitualResult,
        _state: &SymbolicState,
    ) -> Result<ReflectionResult, CodexError> {
        Ok(ReflectionResult {
            ritual_name: ritual_result.ritual_name.clone(),
            timestamp: Utc::now(),
            archetypal_interpretation: format!("The ritual '{}' activated archetypal forces within the psyche, revealing patterns of transformation.", ritual_result.ritual_name),
            symbolic_meaning: format!("Emergent symbols: {} - These represent the unconscious content seeking integration.", ritual_result.emergent_symbols.join(", ")),
            integration_guidance: "Focus on embodying the insights through daily practice and conscious awareness.".to_string(),
            emergent_insights: vec![
                "The ritual has opened new pathways for growth".to_string(),
                "Shadow aspects are ready for integration".to_string(),
            ],
            resonance_analysis: format!("Resonance level of {:.2} indicates moderate to strong energetic alignment.", ritual_result.resonance_level),
            next_steps: vec![
                "Continue with regular meditation practice".to_string(),
                "Journal about the symbols that emerged".to_string(),
            ],
        })
    }

    fn build_reflection_context(
        &self,
        ritual_result: &RitualResult,
        state: &SymbolicState,
    ) -> String {
        format!(
            "Ritual: {}\nSymbols: {}\nState: {}",
            ritual_result.ritual_name,
            ritual_result.emergent_symbols.join(", "),
            state.get_activation_summary()
        )
    }

    pub fn format_reflection_output(&self, reflection: &ReflectionResult) -> String {
        use colored::*;

        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "=".repeat(60).bright_purple()));
        output.push_str(&format!(
            "{}\n",
            format!("🔮 REFLECTION ON: {}", reflection.ritual_name)
                .bright_cyan()
                .bold()
        ));
        output.push_str(&format!("{}\n", "=".repeat(60).bright_purple()));

        output.push_str(&format!(
            "\n{}\n",
            "🏛️  ARCHETYPAL INTERPRETATION".bright_yellow().bold()
        ));
        output.push_str(&format!(
            "{}\n",
            reflection.archetypal_interpretation.white()
        ));

        output.push_str(&format!(
            "\n{}\n",
            "🔯 SYMBOLIC MEANING".bright_magenta().bold()
        ));
        output.push_str(&format!("{}\n", reflection.symbolic_meaning.white()));

        output.push_str(&format!(
            "\n{}\n",
            "🌀 INTEGRATION GUIDANCE".bright_green().bold()
        ));
        output.push_str(&format!("{}\n", reflection.integration_guidance.white()));

        if !reflection.emergent_insights.is_empty() {
            output.push_str(&format!(
                "\n{}\n",
                "💡 EMERGENT INSIGHTS".bright_blue().bold()
            ));
            for insight in &reflection.emergent_insights {
                output.push_str(&format!("  • {}\n", insight.white()));
            }
        }

        output.push_str(&format!(
            "\n{}\n",
            "〰️  RESONANCE ANALYSIS".bright_red().bold()
        ));
        output.push_str(&format!("{}\n", reflection.resonance_analysis.white()));

        if !reflection.next_steps.is_empty() {
            output.push_str(&format!("\n{}\n", "🎯 NEXT STEPS".bright_cyan().bold()));
            for step in &reflection.next_steps {
                output.push_str(&format!("  • {}\n", step.white()));
            }
        }

        output.push_str(&format!("\n{}\n", "=".repeat(60).bright_purple()));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ritual::{ChangeType, CompletionStatus, RitualResult, StateChange};
    use crate::state::{Archetype, Element, Energy, Integration, SymbolicState};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_ritual_result() -> RitualResult {
        RitualResult {
            ritual_name: "shadow_integration".to_string(),
            execution_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            duration_ms: 250,
            symbolic_outputs: {
                let mut outputs = HashMap::new();
                outputs.insert("shadows_integrated".to_string(), serde_json::json!(2));
                outputs
            },
            state_changes: vec![
                StateChange {
                    change_type: ChangeType::Integration,
                    description: "Integrated shadow aspect 'Pride'".to_string(),
                    magnitude: 0.7,
                },
                StateChange {
                    change_type: ChangeType::ArchetypeActivation,
                    description: "Activated Shadow archetype".to_string(),
                    magnitude: 0.8,
                },
            ],
            emergent_symbols: vec!["🌑→🌕".to_string(), "∫∂∇".to_string()],
            completion_status: CompletionStatus::Complete,
            resonance_level: 0.75,
        }
    }

    fn create_test_symbolic_state() -> SymbolicState {
        let mut state = SymbolicState::new();
        
        let mut shadow = Archetype::new("Shadow".to_string(), "Dark aspects".to_string());
        shadow.activation_level = 0.8;
        state.add_archetype(shadow);
        
        let mut fire = Energy::new("Fire".to_string(), 528.0, Element::Fire);
        fire.amplitude = 0.6;
        state.add_energy(fire);
        
        state.add_integration(Integration::new(
            "Shadow Work".to_string(),
            "Embracing darkness".to_string(),
            vec![]
        ));
        
        state
    }

    #[test]
    fn test_reflection_config_default() {
        let config = ReflectionConfig::default();
        
        assert_eq!(config.api_base_url, "https://openrouter.ai/api/v1");
        assert_eq!(config.model, "anthropic/claude-3.5-sonnet");
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, 2000);
        // api_key will be empty if env var not set
    }

    #[test]
    fn test_reflector_creation() {
        let config = ReflectionConfig {
            api_base_url: "https://test-api.com".to_string(),
            api_key: "test-key".to_string(),
            model: "test-model".to_string(),
            temperature: 0.8,
            max_tokens: 1500,
        };
        
        let reflector = Reflector::new(config.clone());
        assert_eq!(reflector.config.api_base_url, config.api_base_url);
        assert_eq!(reflector.config.api_key, config.api_key);
        assert_eq!(reflector.config.model, config.model);
        assert_eq!(reflector.config.temperature, config.temperature);
        assert_eq!(reflector.config.max_tokens, config.max_tokens);
    }

    #[test]
    fn test_reflector_with_defaults() {
        let reflector = Reflector::new_with_defaults();
        assert_eq!(reflector.config.api_base_url, "https://openrouter.ai/api/v1");
        assert_eq!(reflector.config.model, "anthropic/claude-3.5-sonnet");
    }

    #[tokio::test]
    async fn test_mock_reflection_when_no_api_key() {
        let config = ReflectionConfig {
            api_base_url: "https://openrouter.ai/api/v1".to_string(),
            api_key: "".to_string(), // Empty API key
            model: "test-model".to_string(),
            temperature: 0.7,
            max_tokens: 2000,
        };
        
        let reflector = Reflector::new(config);
        let ritual_result = create_test_ritual_result();
        let state = create_test_symbolic_state();
        
        let reflection = reflector.reflect_on_ritual(&ritual_result, &state).await.unwrap();
        
        assert_eq!(reflection.ritual_name, "shadow_integration");
        assert!(reflection.archetypal_interpretation.contains("shadow_integration"));
        assert!(reflection.symbolic_meaning.contains("🌑→🌕"));
        assert!(reflection.symbolic_meaning.contains("∫∂∇"));
        assert!(reflection.resonance_analysis.contains("0.75"));
        assert!(!reflection.emergent_insights.is_empty());
        assert!(!reflection.next_steps.is_empty());
    }

    #[test]
    fn test_build_reflection_context() {
        let reflector = Reflector::new_with_defaults();
        let ritual_result = create_test_ritual_result();
        let state = create_test_symbolic_state();
        
        let context = reflector.build_reflection_context(&ritual_result, &state);
        
        assert!(context.contains("shadow_integration"));
        assert!(context.contains("🌑→🌕"));
        assert!(context.contains("∫∂∇"));
        assert!(context.contains("Archetypes:"));
        assert!(context.contains("Energy:"));
    }

    #[test]
    fn test_parse_ai_reflection_structured_response() {
        let reflector = Reflector::new_with_defaults();
        let ritual_result = create_test_ritual_result();
        
        let ai_response = r#"ARCHETYPAL_INTERPRETATION: The shadow integration ritual represents deep psychological work with the darker aspects of the psyche.

SYMBOLIC_MEANING: The moon transformation symbol represents the journey from darkness to illumination.

INTEGRATION_GUIDANCE: Practice shadow dialogue and active imagination to integrate these insights.

EMERGENT_INSIGHTS: Shadow work opens doorways to authentic power | Integration requires patience and self-compassion | The unconscious reveals its treasures gradually

RESONANCE_ANALYSIS: The resonance level indicates successful engagement with archetypal energies.

NEXT_STEPS: Continue regular shadow work practice | Journal about emerged symbols | Seek guidance from trusted mentors | Practice grounding techniques"#.to_string();
        
        let reflection = reflector.parse_ai_reflection(ai_response, &ritual_result).unwrap();
        
        assert!(reflection.archetypal_interpretation.contains("shadow integration"));
        assert!(reflection.symbolic_meaning.contains("moon transformation"));
        assert!(reflection.integration_guidance.contains("shadow dialogue"));
        assert_eq!(reflection.emergent_insights.len(), 3);
        assert!(reflection.emergent_insights[0].contains("authentic power"));
        assert_eq!(reflection.next_steps.len(), 4);
        assert!(reflection.next_steps[0].contains("shadow work"));
        assert!(reflection.resonance_analysis.contains("resonance level"));
    }

    #[test]
    fn test_parse_ai_reflection_with_fallbacks() {
        let reflector = Reflector::new_with_defaults();
        let ritual_result = create_test_ritual_result();
        
        // Test with incomplete AI response
        let ai_response = r#"ARCHETYPAL_INTERPRETATION: The shadow work was deep.
        
Some unstructured text that doesn't match our format.
        
NEXT_STEPS: Continue the work | Stay grounded"#.to_string();
        
        let reflection = reflector.parse_ai_reflection(ai_response, &ritual_result).unwrap();
        
        assert_eq!(reflection.archetypal_interpretation, "The shadow work was deep.");
        // Should have fallback symbolic meaning
        assert!(reflection.symbolic_meaning.contains("🌑→🌕"));
        assert!(reflection.symbolic_meaning.contains("∫∂∇"));
        // Should have fallback integration guidance
        assert!(reflection.integration_guidance.contains("embodying these insights"));
        assert_eq!(reflection.next_steps.len(), 2);
        assert!(reflection.next_steps[0].contains("Continue the work"));
        // Should have fallback resonance analysis
        assert!(reflection.resonance_analysis.contains("0.75"));
    }

    #[test]
    fn test_parse_ai_reflection_empty_response() {
        let reflector = Reflector::new_with_defaults();
        let ritual_result = create_test_ritual_result();
        
        let ai_response = "".to_string();
        
        let reflection = reflector.parse_ai_reflection(ai_response, &ritual_result).unwrap();
        
        // Should all be fallback values
        assert!(reflection.archetypal_interpretation.contains("shadow_integration"));
        assert!(reflection.symbolic_meaning.contains("🌑→🌕"));
        assert!(reflection.integration_guidance.contains("embodying these insights"));
        assert!(reflection.resonance_analysis.contains("0.75"));
    }

    #[test]
    fn test_create_mock_reflection() {
        let reflector = Reflector::new_with_defaults();
        let ritual_result = create_test_ritual_result();
        let state = create_test_symbolic_state();
        
        let reflection = reflector.create_mock_reflection(&ritual_result, &state).unwrap();
        
        assert_eq!(reflection.ritual_name, "shadow_integration");
        assert!(reflection.archetypal_interpretation.contains("archetypal forces"));
        assert!(reflection.symbolic_meaning.contains("🌑→🌕"));
        assert!(reflection.integration_guidance.contains("daily practice"));
        assert_eq!(reflection.emergent_insights.len(), 2);
        assert_eq!(reflection.next_steps.len(), 2);
        assert!(reflection.resonance_analysis.contains("0.75"));
    }

    #[test]
    fn test_format_reflection_output() {
        let reflector = Reflector::new_with_defaults();
        let ritual_result = create_test_ritual_result();
        let state = create_test_symbolic_state();
        
        let reflection = reflector.create_mock_reflection(&ritual_result, &state).unwrap();
        let output = reflector.format_reflection_output(&reflection);
        
        // Should contain all sections
        assert!(output.contains("🔮 REFLECTION ON: shadow_integration"));
        assert!(output.contains("🏛️  ARCHETYPAL INTERPRETATION"));
        assert!(output.contains("🔯 SYMBOLIC MEANING"));
        assert!(output.contains("🌀 INTEGRATION GUIDANCE"));
        assert!(output.contains("💡 EMERGENT INSIGHTS"));
        assert!(output.contains("〰️  RESONANCE ANALYSIS"));
        assert!(output.contains("🎯 NEXT STEPS"));
        
        // Should contain the actual content
        assert!(output.contains("archetypal forces"));
        assert!(output.contains("🌑→🌕"));
        assert!(output.contains("daily practice"));
        assert!(output.contains("regular meditation"));
        assert!(output.contains("0.75"));
    }

    #[test]
    fn test_reflection_result_serialization() {
        let reflection = ReflectionResult {
            ritual_name: "test_ritual".to_string(),
            timestamp: Utc::now(),
            archetypal_interpretation: "Test interpretation".to_string(),
            symbolic_meaning: "Test symbols".to_string(),
            integration_guidance: "Test guidance".to_string(),
            emergent_insights: vec!["Insight 1".to_string(), "Insight 2".to_string()],
            resonance_analysis: "Test resonance".to_string(),
            next_steps: vec!["Step 1".to_string(), "Step 2".to_string()],
        };
        
        // Test serialization to JSON
        let json = serde_json::to_string(&reflection).unwrap();
        assert!(json.contains("test_ritual"));
        assert!(json.contains("Test interpretation"));
        
        // Test deserialization from JSON
        let deserialized: ReflectionResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.ritual_name, reflection.ritual_name);
        assert_eq!(deserialized.archetypal_interpretation, reflection.archetypal_interpretation);
        assert_eq!(deserialized.emergent_insights.len(), 2);
        assert_eq!(deserialized.next_steps.len(), 2);
    }
}
