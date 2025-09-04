'use client';

import { useQuery } from '@tanstack/react-query';
import { ritualsAPI, Practitioner } from '@/lib/api';
import { LoadingSpinner } from '@/components/ui/loading-spinner';
import { Button } from '@/components/ui/button';
import { 
  BookOpen, 
  Star, 
  Clock, 
  Target, 
  Users, 
  Search,
  Filter,
  Circle,
  Sparkles,
  Flame,
  Droplet,
  Mountain,
  Wind,
  Moon
} from 'lucide-react';

interface RitualLibraryProps {
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

const difficultyColors = {
  beginner: 'text-green-400 bg-green-900/20',
  intermediate: 'text-yellow-400 bg-yellow-900/20',
  advanced: 'text-red-400 bg-red-900/20',
};

const traditionColors = {
  jungian: 'border-blue-500/30 bg-blue-900/10',
  shamanic: 'border-green-500/30 bg-green-900/10',
  hermetic: 'border-yellow-500/30 bg-yellow-900/10',
  elemental: 'border-red-500/30 bg-red-900/10',
  mystical: 'border-purple-500/30 bg-purple-900/10',
  universal: 'border-gray-500/30 bg-gray-900/10',
};

export function RitualLibrary({ user }: RitualLibraryProps) {
  // Fetch ritual catalog
  const { data: rituals = [], isLoading: ritualsLoading } = useQuery({
    queryKey: ['rituals', 'catalog'],
    queryFn: ritualsAPI.getCatalog,
  });

  const renderEnergyRequirements = (requirements: Record<string, number>) => {
    return (
      <div className="flex flex-wrap gap-1">
        {Object.entries(requirements).map(([energy, level]) => {
          const Icon = energyIcons[energy] || Circle;
          return (
            <div
              key={energy}
              className="flex items-center space-x-1 px-2 py-1 bg-slate-700/30 rounded-full text-xs"
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

  if (ritualsLoading) {
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
          Sacred Ritual Library
        </h2>
        <p className="text-slate-400">
          Explore the collection of transformative sacred practices
        </p>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-4 border border-slate-700/50 text-center">
          <BookOpen className="h-8 w-8 text-purple-400 mx-auto mb-2" />
          <div className="text-2xl font-bold text-white">{rituals.length}</div>
          <div className="text-sm text-slate-400">Total Rituals</div>
        </div>
        
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-4 border border-slate-700/50 text-center">
          <Star className="h-8 w-8 text-yellow-400 mx-auto mb-2" />
          <div className="text-2xl font-bold text-white">
            {rituals.length > 0 
              ? (rituals.reduce((sum, r) => sum + r.effectiveness_rating, 0) / rituals.length).toFixed(1)
              : '0.0'
            }
          </div>
          <div className="text-sm text-slate-400">Avg Rating</div>
        </div>
        
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-4 border border-slate-700/50 text-center">
          <Users className="h-8 w-8 text-blue-400 mx-auto mb-2" />
          <div className="text-2xl font-bold text-white">
            {rituals.reduce((sum, r) => sum + r.usage_count, 0)}
          </div>
          <div className="text-sm text-slate-400">Total Uses</div>
        </div>
        
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-4 border border-slate-700/50 text-center">
          <Sparkles className="h-8 w-8 text-green-400 mx-auto mb-2" />
          <div className="text-2xl font-bold text-white">
            {new Set(rituals.map(r => r.tradition)).size}
          </div>
          <div className="text-sm text-slate-400">Traditions</div>
        </div>
      </div>

      {/* Rituals Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
        {rituals.map((ritual) => (
          <div
            key={ritual.id}
            className={`p-6 rounded-lg border transition-all hover:shadow-lg hover:shadow-purple-500/10 ${
              traditionColors[ritual.tradition as keyof typeof traditionColors] || 
              traditionColors.universal
            }`}
          >
            {/* Header */}
            <div className="flex items-start justify-between mb-4">
              <div className="flex-1">
                <h3 className="text-lg font-semibold text-white mb-1">
                  {ritual.name.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase())}
                </h3>
                <div className="flex items-center space-x-2 mb-2">
                  <span 
                    className={`px-2 py-1 rounded-full text-xs font-medium ${
                      difficultyColors[ritual.difficulty_level as keyof typeof difficultyColors] ||
                      'text-slate-400 bg-slate-700/30'
                    }`}
                  >
                    {ritual.difficulty_level}
                  </span>
                  <span className="px-2 py-1 rounded-full text-xs font-medium bg-purple-900/30 text-purple-300">
                    {ritual.tradition}
                  </span>
                </div>
              </div>
              <div className="text-right">
                <div className="flex items-center space-x-1 text-yellow-400 text-sm">
                  <Star className="h-4 w-4 fill-current" />
                  <span>{ritual.effectiveness_rating.toFixed(1)}</span>
                </div>
                <div className="text-xs text-slate-400">
                  {ritual.rating_count} reviews
                </div>
              </div>
            </div>

            {/* Description */}
            <p className="text-slate-400 text-sm mb-4">
              {ritual.description}
            </p>

            {/* Intent */}
            <div className="mb-4">
              <h4 className="text-xs font-medium text-slate-300 mb-1 flex items-center space-x-1">
                <Target className="h-3 w-3" />
                <span>Sacred Intent</span>
              </h4>
              <p className="text-xs text-slate-400 italic">
                "{ritual.intent}"
              </p>
            </div>

            {/* Required Archetypes */}
            {ritual.required_archetypes.length > 0 && (
              <div className="mb-4">
                <h4 className="text-xs font-medium text-slate-300 mb-2">
                  Required Archetypes
                </h4>
                <div className="flex flex-wrap gap-1">
                  {ritual.required_archetypes.map((archetype) => (
                    <span
                      key={archetype}
                      className="px-2 py-1 bg-slate-700/50 rounded-full text-xs text-slate-300"
                    >
                      {archetype}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {/* Energy Requirements */}
            {Object.keys(ritual.energy_requirements).length > 0 && (
              <div className="mb-4">
                <h4 className="text-xs font-medium text-slate-300 mb-2">
                  Energy Requirements
                </h4>
                {renderEnergyRequirements(ritual.energy_requirements)}
              </div>
            )}

            {/* Usage Stats */}
            <div className="flex items-center justify-between text-xs text-slate-500 mb-4">
              <div className="flex items-center space-x-1">
                <Clock className="h-3 w-3" />
                <span>{ritual.usage_count} executions</span>
              </div>
              <div className="flex items-center space-x-1">
                <Users className="h-3 w-3" />
                <span>{ritual.is_public ? 'Community' : 'Private'}</span>
              </div>
            </div>

            {/* Tags */}
            {ritual.tags.length > 0 && (
              <div className="mb-4">
                <div className="flex flex-wrap gap-1">
                  {ritual.tags.slice(0, 3).map((tag) => (
                    <span
                      key={tag}
                      className="px-2 py-1 bg-purple-900/30 text-purple-300 rounded-full text-xs"
                    >
                      #{tag}
                    </span>
                  ))}
                  {ritual.tags.length > 3 && (
                    <span className="px-2 py-1 bg-slate-700/30 text-slate-400 rounded-full text-xs">
                      +{ritual.tags.length - 3} more
                    </span>
                  )}
                </div>
              </div>
            )}

            {/* Actions */}
            <div className="flex items-center space-x-2">
              <Button variant="sacred" size="sm" className="flex-1">
                <Sparkles className="h-3 w-3 mr-1" />
                Execute Ritual
              </Button>
              <Button variant="outline" size="sm">
                <BookOpen className="h-3 w-3" />
              </Button>
            </div>
          </div>
        ))}
      </div>

      {rituals.length === 0 && (
        <div className="text-center py-12">
          <BookOpen className="h-16 w-16 text-slate-600 mx-auto mb-4" />
          <h3 className="text-lg font-medium text-white mb-2">
            No Rituals Available
          </h3>
          <p className="text-slate-400">
            The sacred library is currently being prepared for your journey
          </p>
        </div>
      )}
    </div>
  );
}