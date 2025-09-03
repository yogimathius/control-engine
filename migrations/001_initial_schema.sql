-- Sacred Database Schema for Codex Control Engine
-- This schema supports the archetypal transformation system

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Practitioners and their sacred profiles
CREATE TABLE practitioners (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    spiritual_name VARCHAR(100), -- chosen sacred name
    archetypal_preferences JSONB DEFAULT '{}', -- preferred archetypes
    energy_alignments JSONB DEFAULT '{}', -- elemental affinities
    privacy_level VARCHAR(20) DEFAULT 'private', -- private, community, public
    sacred_path VARCHAR(100), -- jungian, shamanic, hermetic, etc.
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Symbolic states with archetypal data
CREATE TABLE archetypal_states (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    practitioner_id UUID NOT NULL REFERENCES practitioners(id) ON DELETE CASCADE,
    state_data JSONB NOT NULL, -- full symbolic state
    archetypes JSONB NOT NULL, -- active archetypes and their strength
    energies JSONB NOT NULL, -- elemental energy levels
    integrations JSONB DEFAULT '[]', -- completed integrations
    symbols JSONB DEFAULT '[]', -- emergent symbols
    transformations JSONB DEFAULT '[]', -- active processes
    state_hash VARCHAR(64), -- cryptographic state verification
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Ritual definitions and WASM modules
CREATE TABLE sacred_rituals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    intent TEXT NOT NULL, -- ritual purpose
    tradition VARCHAR(100) DEFAULT 'universal', -- jungian, shamanic, alchemical, etc.
    difficulty_level VARCHAR(20) DEFAULT 'beginner',
    required_archetypes JSONB DEFAULT '[]',
    energy_requirements JSONB DEFAULT '{}',
    wasm_module_data BYTEA, -- WASM binary data
    wasm_module_hash VARCHAR(64), -- verification hash
    module_language VARCHAR(50), -- rust, c, assemblyscript, etc.
    author_id UUID REFERENCES practitioners(id),
    usage_count INTEGER DEFAULT 0,
    effectiveness_rating DECIMAL(3,2) DEFAULT 0.0,
    rating_count INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT false,
    tags JSONB DEFAULT '[]', -- searchable tags
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Ritual execution history
CREATE TABLE ritual_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
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
    ritual_parameters JSONB DEFAULT '{}', -- parameters used in execution
    intention TEXT, -- practitioner's stated intention
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- AI reflections and archetypal interpretations
CREATE TABLE oracle_insights (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID REFERENCES ritual_sessions(id) ON DELETE CASCADE,
    practitioner_id UUID REFERENCES practitioners(id) ON DELETE CASCADE,
    insight_type VARCHAR(50) DEFAULT 'interpretation', -- interpretation, guidance, warning, celebration
    archetypal_analysis JSONB, -- detailed symbolic interpretation
    integration_suggestions JSONB, -- practical next steps
    symbolic_emergence JSONB, -- new symbols that appeared
    oracle_model VARCHAR(100), -- which AI model provided insight
    confidence_score DECIMAL(3,2) DEFAULT 0.0,
    full_response TEXT, -- complete AI response
    query_context JSONB, -- context provided to AI
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Collective ritual spaces (Future Phase)
CREATE TABLE collective_spaces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    purpose TEXT NOT NULL,
    tradition VARCHAR(100),
    facilitator_id UUID REFERENCES practitioners(id),
    participant_limit INTEGER DEFAULT 12, -- sacred number
    current_participants INTEGER DEFAULT 0,
    shared_state JSONB DEFAULT '{}', -- collective symbolic state
    ritual_schedule JSONB, -- when collective rituals occur
    access_level VARCHAR(20) DEFAULT 'invite', -- open, invite, private
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Ritual ratings and reviews
CREATE TABLE ritual_reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ritual_id UUID NOT NULL REFERENCES sacred_rituals(id) ON DELETE CASCADE,
    practitioner_id UUID NOT NULL REFERENCES practitioners(id) ON DELETE CASCADE,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 10),
    review_text TEXT,
    transformation_achieved BOOLEAN DEFAULT false,
    would_recommend BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(ritual_id, practitioner_id) -- One review per practitioner per ritual
);

-- Indexes for performance
CREATE INDEX idx_practitioners_email ON practitioners(email);
CREATE INDEX idx_archetypal_states_practitioner ON archetypal_states(practitioner_id);
CREATE INDEX idx_archetypal_states_created ON archetypal_states(created_at);
CREATE INDEX idx_ritual_sessions_practitioner ON ritual_sessions(practitioner_id);
CREATE INDEX idx_ritual_sessions_ritual ON ritual_sessions(ritual_id);
CREATE INDEX idx_ritual_sessions_created ON ritual_sessions(created_at);
CREATE INDEX idx_sacred_rituals_public ON sacred_rituals(is_public) WHERE is_public = true;
CREATE INDEX idx_sacred_rituals_author ON sacred_rituals(author_id);
CREATE INDEX idx_sacred_rituals_tradition ON sacred_rituals(tradition);
CREATE INDEX idx_oracle_insights_session ON oracle_insights(session_id);
CREATE INDEX idx_oracle_insights_practitioner ON oracle_insights(practitioner_id);

-- Insert foundational rituals
INSERT INTO sacred_rituals (name, description, intent, tradition, difficulty_level, required_archetypes, energy_requirements, is_public) VALUES
(
    'shadow_integration',
    'A deep ritual for integrating shadow aspects of the psyche through archetypal activation',
    'To bring unconscious shadow material into conscious awareness for integration and wholeness',
    'jungian',
    'intermediate',
    '["Shadow"]',
    '{"Shadow": 0.3, "Earth": 0.2}',
    true
),
(
    'archetype_invocation',
    'Sacred invocation ritual to activate dormant archetypal forces within the psyche',
    'To awaken and align with specific archetypal energies for personal transformation',
    'universal',
    'beginner',
    '[]',
    '{"Fire": 0.2, "Air": 0.1}',
    true
),
(
    'energy_attunement',
    'Harmonic ritual for balancing and attuning elemental energies within the sacred self',
    'To achieve energetic balance and harmonic resonance across all elemental frequencies',
    'elemental',
    'beginner',
    '["Sage"]',
    '{"Fire": 0.1, "Water": 0.1, "Earth": 0.1, "Air": 0.1}',
    true
),
(
    'void_contemplation',
    'Deep contemplative ritual for entering the sacred void and connecting with infinite potential',
    'To dissolve temporary identity and connect with the eternal, formless essence',
    'mystical',
    'advanced',
    '["Sage", "Mystic"]',
    '{"Void": 0.5}',
    true
);

-- Create trigger for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_practitioners_updated_at BEFORE UPDATE ON practitioners FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_sacred_rituals_updated_at BEFORE UPDATE ON sacred_rituals FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_collective_spaces_updated_at BEFORE UPDATE ON collective_spaces FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();