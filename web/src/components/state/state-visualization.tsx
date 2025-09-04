'use client';

import { useQuery } from '@tanstack/react-query';
import { stateAPI, Practitioner, ArchetypalState } from '@/lib/api';
import { LoadingSpinner } from '@/components/ui/loading-spinner';
import { 
  Circle, 
  Flame, 
  Droplet, 
  Mountain, 
  Wind, 
  Moon,
  Calendar,
  TrendingUp,
  Hash
} from 'lucide-react';

interface StateVisualizationProps {
  user: Practitioner;
}

const energyIcons: Record<string, any> = {
  Fire: Flame,
  Water: Droplet,
  Earth: Mountain,
  Air: Wind,
  Void: Moon,
  Shadow: Circle,
};

const archetypeDescriptions: Record<string, string> = {
  Sage: 'The wise teacher who seeks truth and understanding',
  Warrior: 'The brave protector who faces challenges with courage',
  Lover: 'The passionate heart who seeks connection and beauty',
  Innocent: 'The pure soul who sees wonder in the world',
  Explorer: 'The adventurous spirit who seeks new experiences',
  Creator: 'The imaginative force who brings new things into being',
  Ruler: 'The responsible leader who creates order and structure',
  Caregiver: 'The nurturing soul who serves and protects others',
  Magician: 'The transformer who makes dreams into reality',
  Outlaw: 'The rebel who challenges the status quo',
  Hero: 'The champion who overcomes great trials',
  Jester: 'The playful spirit who brings joy and lightness',
  Shadow: 'The hidden aspects seeking integration and wholeness',
  Mystic: 'The seeker of transcendent truth and spiritual unity',
};

export function StateVisualization({ user }: StateVisualizationProps) {
  // Fetch current state
  const { data: currentState, isLoading: currentLoading } = useQuery({
    queryKey: ['state', 'current'],
    queryFn: stateAPI.getCurrentState,
  });

  // Fetch state history
  const { data: stateHistory = [], isLoading: historyLoading } = useQuery({
    queryKey: ['state', 'history'],
    queryFn: stateAPI.getStateHistory,
  });

  if (currentLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <LoadingSpinner size="large" />
      </div>
    );
  }

  if (!currentState) {
    return (
      <div className="text-center py-12">
        <Circle className="h-16 w-16 text-slate-600 mx-auto mb-4" />
        <h3 className="text-lg font-medium text-white mb-2">
          No Sacred State Found
        </h3>
        <p className="text-slate-400">
          Execute your first ritual to begin your archetypal journey
        </p>
      </div>
    );
  }

  const renderArchetypeCircle = (archetype: string, strength: number) => {
    const radius = 50;
    const circumference = 2 * Math.PI * radius;
    const strokeDasharray = circumference;
    const strokeDashoffset = circumference - (strength * circumference);

    return (
      <div className="relative flex flex-col items-center">
        <div className="relative">
          <svg className="w-24 h-24 transform -rotate-90" viewBox="0 0 120 120">
            <circle
              cx="60"
              cy="60"
              r={radius}
              stroke="rgb(71, 85, 105)"
              strokeWidth="8"
              fill="none"
            />
            <circle
              cx="60"
              cy="60"
              r={radius}
              stroke="rgb(147, 51, 234)"
              strokeWidth="8"
              fill="none"
              strokeDasharray={strokeDasharray}
              strokeDashoffset={strokeDashoffset}
              strokeLinecap="round"
              className="transition-all duration-500"
            />
          </svg>
          <div className="absolute inset-0 flex items-center justify-center">
            <span className="text-white font-semibold text-sm">
              {Math.round(strength * 100)}%
            </span>
          </div>
        </div>
        <div className="mt-2 text-center">
          <h4 className="text-sm font-medium text-white">{archetype}</h4>
          <p className="text-xs text-slate-400 max-w-32">
            {archetypeDescriptions[archetype] || 'Sacred archetype'}
          </p>
        </div>
      </div>
    );
  };

  const renderEnergyBar = (energy: string, level: number) => {
    const Icon = energyIcons[energy] || Circle;
    
    return (
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <Icon className="h-5 w-5 text-purple-400" />
            <span className="text-white font-medium">{energy}</span>
          </div>
          <span className="text-purple-400 font-semibold">
            {Math.round(level * 100)}%
          </span>
        </div>
        <div className="w-full h-3 bg-slate-700 rounded-full overflow-hidden">
          <div
            className="h-full bg-gradient-to-r from-purple-600 to-blue-500 rounded-full transition-all duration-500"
            style={{ width: `${Math.min(level * 100, 100)}%` }}
          />
        </div>
      </div>
    );
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="text-center">
        <h2 className="text-3xl font-bold text-white mb-2">
          Archetypal State Visualization
        </h2>
        <p className="text-slate-400">
          Your current sacred configuration and transformation journey
        </p>
      </div>

      {/* State Overview */}
      <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-6 border border-slate-700/50">
        <div className="flex items-center justify-between mb-6">
          <h3 className="text-xl font-semibold text-white">
            Current Sacred State
          </h3>
          <div className="flex items-center space-x-4 text-sm text-slate-400">
            <div className="flex items-center space-x-1">
              <Calendar className="h-4 w-4" />
              <span>{new Date(currentState.created_at).toLocaleDateString()}</span>
            </div>
            <div className="flex items-center space-x-1">
              <Hash className="h-4 w-4" />
              <span className="font-mono text-xs">
                {currentState.state_hash?.slice(0, 8) || 'N/A'}
              </span>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Archetypes */}
          <div>
            <h4 className="text-lg font-medium text-white mb-4">
              Active Archetypes
            </h4>
            <div className="grid grid-cols-2 gap-4">
              {Object.entries(currentState.archetypes).map(([archetype, strength]) => (
                <div key={archetype}>
                  {renderArchetypeCircle(archetype, Number(strength))}
                </div>
              ))}
            </div>
          </div>

          {/* Energies */}
          <div>
            <h4 className="text-lg font-medium text-white mb-4">
              Elemental Energies
            </h4>
            <div className="space-y-4">
              {Object.entries(currentState.energies).map(([energy, level]) => (
                <div key={energy}>
                  {renderEnergyBar(energy, Number(level))}
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Integrations */}
      {currentState.integrations.length > 0 && (
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-6 border border-slate-700/50">
          <h3 className="text-xl font-semibold text-white mb-4">
            Completed Integrations
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {currentState.integrations.map((integration: any, index) => (
              <div
                key={index}
                className="p-4 bg-green-900/20 border border-green-700/50 rounded-lg"
              >
                <h4 className="font-medium text-green-300 mb-2">
                  {integration.type || 'Sacred Integration'}
                </h4>
                <p className="text-sm text-green-400">
                  Depth: {Math.round((integration.depth || 0) * 100)}%
                </p>
                {integration.description && (
                  <p className="text-xs text-green-200 mt-2">
                    {integration.description}
                  </p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Symbols */}
      {currentState.symbols.length > 0 && (
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-6 border border-slate-700/50">
          <h3 className="text-xl font-semibold text-white mb-4">
            Emergent Symbols
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {currentState.symbols.map((symbol: any, index) => (
              <div
                key={index}
                className="p-4 bg-purple-900/20 border border-purple-700/50 rounded-lg text-center"
              >
                <div className="text-2xl mb-2">{symbol.glyph || '🔮'}</div>
                <h4 className="font-medium text-purple-300 mb-1">
                  {symbol.name || `Symbol ${index + 1}`}
                </h4>
                {symbol.meaning && (
                  <p className="text-xs text-purple-200">
                    {symbol.meaning}
                  </p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* State History */}
      {!historyLoading && stateHistory.length > 1 && (
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-6 border border-slate-700/50">
          <h3 className="text-xl font-semibold text-white mb-4 flex items-center space-x-2">
            <TrendingUp className="h-5 w-5" />
            <span>Transformation Journey</span>
          </h3>
          <div className="space-y-3">
            {stateHistory.slice(0, 5).map((state, index) => (
              <div
                key={state.id}
                className={`p-3 rounded border ${
                  index === 0
                    ? 'bg-purple-900/30 border-purple-500/50'
                    : 'bg-slate-700/30 border-slate-600/50'
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="text-sm text-white">
                    {index === 0 ? 'Current State' : `${index + 1} states ago`}
                  </div>
                  <div className="text-xs text-slate-400">
                    {new Date(state.created_at).toLocaleDateString()}
                  </div>
                </div>
                <div className="flex items-center space-x-4 mt-2 text-xs">
                  <span className="text-slate-400">
                    {Object.keys(state.archetypes).length} archetypes
                  </span>
                  <span className="text-slate-400">
                    {Object.keys(state.energies).length} energies
                  </span>
                  <span className="text-slate-400">
                    {state.integrations.length} integrations
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}