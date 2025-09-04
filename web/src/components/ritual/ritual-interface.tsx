'use client';

import { useState, useEffect } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { ritualsAPI, stateAPI, SacredRitual, Practitioner, RitualSession } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { LoadingSpinner } from '@/components/ui/loading-spinner';
import { Input } from '@/components/ui/input';
import { toast } from 'react-hot-toast';
import { 
  Play, 
  Star, 
  Clock, 
  Target, 
  Sparkles, 
  Circle,
  Flame,
  Droplet,
  Mountain,
  Wind,
  Moon
} from 'lucide-react';

interface RitualInterfaceProps {
  user: Practitioner;
}

const traditionColors = {
  jungian: 'from-blue-500 to-purple-600',
  shamanic: 'from-green-500 to-emerald-600',
  hermetic: 'from-yellow-500 to-orange-600',
  elemental: 'from-red-500 to-pink-600',
  mystical: 'from-purple-500 to-indigo-600',
  universal: 'from-gray-500 to-slate-600',
};

const difficultyIcons = {
  beginner: Circle,
  intermediate: Star,
  advanced: Sparkles,
};

const energyIcons: Record<string, React.ElementType> = {
  Fire: Flame,
  Water: Droplet,
  Earth: Mountain,
  Air: Wind,
  Void: Moon,
  Shadow: Circle,
};

export function RitualInterface({ user }: RitualInterfaceProps) {
  const [selectedRitual, setSelectedRitual] = useState<SacredRitual | null>(null);
  const [intention, setIntention] = useState('');
  const [isExecuting, setIsExecuting] = useState(false);
  const [lastSession, setLastSession] = useState<RitualSession | null>(null);

  const queryClient = useQueryClient();

  // Fetch available rituals
  const { data: rituals = [], isLoading: ritualsLoading } = useQuery({
    queryKey: ['rituals', 'catalog'],
    queryFn: ritualsAPI.getCatalog,
  });

  // Fetch current state
  const { data: currentState, isLoading: stateLoading } = useQuery({
    queryKey: ['state', 'current'],
    queryFn: stateAPI.getCurrentState,
  });

  // Execute ritual mutation
  const executeRitualMutation = useMutation({
    mutationFn: (data: { ritualId: string; intention: string }) =>
      ritualsAPI.executeRitual(data.ritualId, {}, data.intention),
    onSuccess: (session) => {
      setLastSession(session);
      queryClient.invalidateQueries({ queryKey: ['state'] });
      toast.success('Ritual completed successfully! State transformation achieved.');
      setIsExecuting(false);
      setIntention('');
    },
    onError: (error) => {
      toast.error('Ritual execution failed. Please try again.');
      setIsExecuting(false);
    },
  });

  const handleExecuteRitual = async () => {
    if (!selectedRitual) return;

    setIsExecuting(true);
    executeRitualMutation.mutate({
      ritualId: selectedRitual.id,
      intention: intention || selectedRitual.intent,
    });
  };

  const renderEnergyRequirements = (requirements: Record<string, number>) => {
    return (
      <div className="flex flex-wrap gap-2">
        {Object.entries(requirements).map(([energy, level]) => {
          const Icon = energyIcons[energy] || Circle;
          return (
            <div
              key={energy}
              className="flex items-center space-x-1 px-2 py-1 bg-slate-700/50 rounded-full text-xs"
            >
              <Icon className="h-3 w-3 text-purple-400" />
              <span className="text-slate-300">{energy}</span>
              <span className="text-slate-400">
                {Math.round(level * 100)}%
              </span>
            </div>
          );
        })}
      </div>
    );
  };

  if (ritualsLoading || stateLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <LoadingSpinner size="large" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="text-center">
        <h2 className="text-3xl font-bold text-white mb-2">
          Sacred Ritual Execution
        </h2>
        <p className="text-slate-400">
          Choose a ritual to transform your archetypal state
        </p>
      </div>

      {/* Current State Summary */}
      {currentState && (
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-6 border border-slate-700/50">
          <h3 className="text-lg font-semibold text-white mb-4">
            Current Archetypal State
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <h4 className="text-sm font-medium text-slate-300 mb-2">
                Active Archetypes
              </h4>
              <div className="space-y-2">
                {Object.entries(currentState.archetypes).map(([archetype, strength]) => (
                  <div
                    key={archetype}
                    className="flex items-center justify-between text-sm"
                  >
                    <span className="text-slate-400">{archetype}</span>
                    <div className="flex items-center space-x-2">
                      <div className="w-16 h-2 bg-slate-700 rounded-full">
                        <div
                          className="h-full bg-purple-500 rounded-full transition-all"
                          style={{ width: `${Math.min(Number(strength) * 100, 100)}%` }}
                        />
                      </div>
                      <span className="text-purple-400 text-xs w-10 text-right">
                        {Math.round(Number(strength) * 100)}%
                      </span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
            <div>
              <h4 className="text-sm font-medium text-slate-300 mb-2">
                Energy Levels
              </h4>
              <div className="space-y-2">
                {Object.entries(currentState.energies).map(([energy, level]) => {
                  const Icon = energyIcons[energy] || Circle;
                  return (
                    <div
                      key={energy}
                      className="flex items-center justify-between text-sm"
                    >
                      <div className="flex items-center space-x-2">
                        <Icon className="h-4 w-4 text-purple-400" />
                        <span className="text-slate-400">{energy}</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <div className="w-16 h-2 bg-slate-700 rounded-full">
                          <div
                            className="h-full bg-blue-500 rounded-full transition-all"
                            style={{ width: `${Math.min(Number(level) * 100, 100)}%` }}
                          />
                        </div>
                        <span className="text-blue-400 text-xs w-10 text-right">
                          {Math.round(Number(level) * 100)}%
                        </span>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Ritual Selection */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Available Rituals */}
        <div className="space-y-4">
          <h3 className="text-xl font-semibold text-white">
            Available Sacred Rituals
          </h3>
          <div className="space-y-3">
            {rituals.map((ritual) => {
              const DifficultyIcon = difficultyIcons[ritual.difficulty_level as keyof typeof difficultyIcons];
              const isSelected = selectedRitual?.id === ritual.id;

              return (
                <div
                  key={ritual.id}
                  onClick={() => setSelectedRitual(ritual)}
                  className={`p-4 rounded-lg border transition-all cursor-pointer ${
                    isSelected
                      ? 'bg-purple-900/30 border-purple-500/50 shadow-lg shadow-purple-500/20'
                      : 'bg-slate-800/30 border-slate-700/50 hover:bg-slate-800/50 hover:border-slate-600'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="flex items-center space-x-2 mb-2">
                        <h4 className="font-medium text-white">
                          {ritual.name.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase())}
                        </h4>
                        {DifficultyIcon && (
                          <DifficultyIcon className="h-4 w-4 text-purple-400" />
                        )}
                      </div>
                      <p className="text-sm text-slate-400 mb-3">
                        {ritual.description}
                      </p>
                      <div className="flex items-center space-x-4 text-xs text-slate-500">
                        <div className="flex items-center space-x-1">
                          <Target className="h-3 w-3" />
                          <span>{ritual.tradition}</span>
                        </div>
                        <div className="flex items-center space-x-1">
                          <Star className="h-3 w-3" />
                          <span>{ritual.effectiveness_rating.toFixed(1)}</span>
                        </div>
                        <div className="flex items-center space-x-1">
                          <Clock className="h-3 w-3" />
                          <span>{ritual.usage_count} uses</span>
                        </div>
                      </div>
                    </div>
                  </div>
                  {Object.keys(ritual.energy_requirements).length > 0 && (
                    <div className="mt-3">
                      <p className="text-xs text-slate-500 mb-1">Energy Requirements:</p>
                      {renderEnergyRequirements(ritual.energy_requirements)}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>

        {/* Ritual Execution Panel */}
        <div className="space-y-4">
          <h3 className="text-xl font-semibold text-white">
            Ritual Execution
          </h3>
          
          {selectedRitual ? (
            <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-6 border border-slate-700/50 space-y-6">
              <div>
                <h4 className="font-medium text-white mb-2">
                  {selectedRitual.name.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase())}
                </h4>
                <p className="text-sm text-slate-400 mb-4">
                  {selectedRitual.intent}
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Sacred Intention (Optional)
                </label>
                <Input
                  value={intention}
                  onChange={(e) => setIntention(e.target.value)}
                  placeholder="Enter your personal intention for this ritual..."
                  className="mb-2"
                />
                <p className="text-xs text-slate-500">
                  Leave blank to use the ritual's default intention
                </p>
              </div>

              <Button
                onClick={handleExecuteRitual}
                variant="sacred"
                className="w-full"
                disabled={isExecuting}
              >
                {isExecuting ? (
                  <div className="flex items-center space-x-2">
                    <LoadingSpinner size="small" />
                    <span>Executing Sacred Ritual...</span>
                  </div>
                ) : (
                  <div className="flex items-center space-x-2">
                    <Play className="h-4 w-4" />
                    <span>Execute Ritual</span>
                  </div>
                )}
              </Button>
            </div>
          ) : (
            <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-6 border border-slate-700/50 text-center">
              <Sparkles className="h-12 w-12 text-slate-600 mx-auto mb-4" />
              <p className="text-slate-400">
                Select a ritual from the list to begin your sacred transformation
              </p>
            </div>
          )}

          {/* Last Session Results */}
          {lastSession && (
            <div className="bg-green-900/20 backdrop-blur-sm rounded-lg p-6 border border-green-700/50">
              <h4 className="font-medium text-white mb-2 flex items-center space-x-2">
                <Sparkles className="h-4 w-4 text-green-400" />
                <span>Ritual Completed</span>
              </h4>
              <p className="text-sm text-green-300 mb-2">
                Your archetypal state has been transformed through sacred ritual.
              </p>
              {lastSession.transformation_intensity && (
                <p className="text-xs text-green-400">
                  Transformation intensity: {Math.round((lastSession.transformation_intensity || 0) * 100)}%
                </p>
              )}
              {lastSession.ai_interpretation && (
                <div className="mt-3 p-3 bg-slate-800/50 rounded text-sm">
                  <p className="text-slate-300">{lastSession.ai_interpretation}</p>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}