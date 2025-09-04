'use client';

import { useState } from 'react';
import { Practitioner } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { 
  User, 
  Mail, 
  Calendar, 
  Heart, 
  Shield, 
  Edit,
  Save,
  X
} from 'lucide-react';

interface ProfileSectionProps {
  user: Practitioner;
}

const sacredPaths = [
  'Jungian Psychology',
  'Shamanic Practice', 
  'Hermetic Tradition',
  'Buddhist Mindfulness',
  'Sufi Mysticism',
  'Celtic Spirituality',
  'Yogic Philosophy',
  'Taoist Wisdom',
  'Christian Mysticism',
  'Universal Sacred Path',
];

const privacyLevels = [
  { value: 'private', label: 'Private', description: 'Only you can see your profile and activities' },
  { value: 'community', label: 'Community', description: 'Sacred community members can see your profile' },
  { value: 'public', label: 'Public', description: 'Anyone can view your sacred journey' },
];

export function ProfileSection({ user }: ProfileSectionProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [editData, setEditData] = useState({
    spiritual_name: user.spiritual_name || '',
    sacred_path: user.sacred_path || '',
    privacy_level: user.privacy_level,
  });

  const handleSave = () => {
    // TODO: Implement profile update API call
    console.log('Saving profile:', editData);
    setIsEditing(false);
  };

  const handleCancel = () => {
    setEditData({
      spiritual_name: user.spiritual_name || '',
      sacred_path: user.sacred_path || '',
      privacy_level: user.privacy_level,
    });
    setIsEditing(false);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="text-center">
        <h2 className="text-3xl font-bold text-white mb-2">
          Sacred Profile
        </h2>
        <p className="text-slate-400">
          Your spiritual identity and transformation preferences
        </p>
      </div>

      {/* Profile Card */}
      <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg border border-slate-700/50 overflow-hidden">
        {/* Header Section */}
        <div className="bg-gradient-to-r from-purple-600/20 to-blue-600/20 p-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <div className="w-16 h-16 bg-purple-600 rounded-full flex items-center justify-center">
                <User className="h-8 w-8 text-white" />
              </div>
              <div>
                <h3 className="text-xl font-semibold text-white">
                  {user.spiritual_name || user.email.split('@')[0]}
                </h3>
                <p className="text-slate-300">
                  {user.sacred_path || 'Sacred Practitioner'}
                </p>
                <p className="text-xs text-slate-400 mt-1">
                  Joined {new Date(user.created_at).toLocaleDateString()}
                </p>
              </div>
            </div>
            
            <Button
              variant="outline"
              size="sm"
              onClick={() => setIsEditing(!isEditing)}
            >
              {isEditing ? (
                <X className="h-4 w-4 mr-1" />
              ) : (
                <Edit className="h-4 w-4 mr-1" />
              )}
              {isEditing ? 'Cancel' : 'Edit'}
            </Button>
          </div>
        </div>

        {/* Profile Details */}
        <div className="p-6 space-y-6">
          {/* Basic Information */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 className="text-sm font-medium text-slate-300 mb-3 flex items-center space-x-2">
                <User className="h-4 w-4" />
                <span>Identity</span>
              </h4>
              
              {isEditing ? (
                <div className="space-y-3">
                  <div>
                    <label className="block text-xs font-medium text-slate-400 mb-1">
                      Spiritual Name
                    </label>
                    <Input
                      value={editData.spiritual_name}
                      onChange={(e) => setEditData(prev => ({ ...prev, spiritual_name: e.target.value }))}
                      placeholder="Your chosen sacred name"
                    />
                  </div>
                  <div>
                    <label className="block text-xs font-medium text-slate-400 mb-1">
                      Sacred Path
                    </label>
                    <select
                      value={editData.sacred_path}
                      onChange={(e) => setEditData(prev => ({ ...prev, sacred_path: e.target.value }))}
                      className="w-full h-10 px-3 py-2 bg-slate-800/50 border border-slate-700 rounded-md text-slate-100 text-sm focus:outline-none focus:ring-2 focus:ring-purple-500"
                    >
                      <option value="">Choose your path...</option>
                      {sacredPaths.map((path) => (
                        <option key={path} value={path}>
                          {path}
                        </option>
                      ))}
                    </select>
                  </div>
                </div>
              ) : (
                <div className="space-y-3 text-sm">
                  <div className="flex items-center justify-between py-2 border-b border-slate-700/50">
                    <span className="text-slate-400">Email</span>
                    <span className="text-white">{user.email}</span>
                  </div>
                  <div className="flex items-center justify-between py-2 border-b border-slate-700/50">
                    <span className="text-slate-400">Spiritual Name</span>
                    <span className="text-white">
                      {user.spiritual_name || 'Not set'}
                    </span>
                  </div>
                  <div className="flex items-center justify-between py-2 border-b border-slate-700/50">
                    <span className="text-slate-400">Sacred Path</span>
                    <span className="text-white">
                      {user.sacred_path || 'Exploring...'}
                    </span>
                  </div>
                </div>
              )}
            </div>

            <div>
              <h4 className="text-sm font-medium text-slate-300 mb-3 flex items-center space-x-2">
                <Shield className="h-4 w-4" />
                <span>Privacy & Preferences</span>
              </h4>
              
              {isEditing ? (
                <div>
                  <label className="block text-xs font-medium text-slate-400 mb-2">
                    Privacy Level
                  </label>
                  <div className="space-y-2">
                    {privacyLevels.map((level) => (
                      <label key={level.value} className="flex items-start space-x-3 cursor-pointer">
                        <input
                          type="radio"
                          value={level.value}
                          checked={editData.privacy_level === level.value}
                          onChange={(e) => setEditData(prev => ({ ...prev, privacy_level: e.target.value }))}
                          className="mt-1 text-purple-600 focus:ring-purple-500"
                        />
                        <div className="flex-1">
                          <div className="text-sm font-medium text-white">
                            {level.label}
                          </div>
                          <div className="text-xs text-slate-400">
                            {level.description}
                          </div>
                        </div>
                      </label>
                    ))}
                  </div>
                </div>
              ) : (
                <div className="space-y-3 text-sm">
                  <div className="flex items-center justify-between py-2 border-b border-slate-700/50">
                    <span className="text-slate-400">Privacy</span>
                    <span className="text-white capitalize">
                      {user.privacy_level}
                    </span>
                  </div>
                  <div className="flex items-center justify-between py-2 border-b border-slate-700/50">
                    <span className="text-slate-400">Member Since</span>
                    <span className="text-white">
                      {new Date(user.created_at).toLocaleDateString('en-US', {
                        year: 'numeric',
                        month: 'long',
                        day: 'numeric'
                      })}
                    </span>
                  </div>
                </div>
              )}
            </div>
          </div>

          {/* Archetypal Preferences */}
          <div>
            <h4 className="text-sm font-medium text-slate-300 mb-3 flex items-center space-x-2">
              <Heart className="h-4 w-4" />
              <span>Archetypal Preferences</span>
            </h4>
            
            {Object.keys(user.archetypal_preferences).length > 0 ? (
              <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
                {Object.entries(user.archetypal_preferences).map(([archetype, preference]) => (
                  <div key={archetype} className="p-3 bg-purple-900/20 border border-purple-700/30 rounded-lg">
                    <div className="text-sm font-medium text-purple-300">
                      {archetype}
                    </div>
                    <div className="text-xs text-purple-200">
                      Affinity: {String(preference)}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-center py-6 text-slate-400">
                <Heart className="h-8 w-8 mx-auto mb-2 text-slate-600" />
                <p className="text-sm">
                  Your archetypal preferences will develop as you engage with rituals
                </p>
              </div>
            )}
          </div>

          {/* Energy Alignments */}
          <div>
            <h4 className="text-sm font-medium text-slate-300 mb-3 flex items-center space-x-2">
              <Calendar className="h-4 w-4" />
              <span>Energy Alignments</span>
            </h4>
            
            {Object.keys(user.energy_alignments).length > 0 ? (
              <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                {Object.entries(user.energy_alignments).map(([energy, alignment]) => (
                  <div key={energy} className="p-3 bg-blue-900/20 border border-blue-700/30 rounded-lg">
                    <div className="text-sm font-medium text-blue-300">
                      {energy}
                    </div>
                    <div className="text-xs text-blue-200">
                      {String(alignment)}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-center py-6 text-slate-400">
                <Calendar className="h-8 w-8 mx-auto mb-2 text-slate-600" />
                <p className="text-sm">
                  Energy alignments will emerge through your ritual practice
                </p>
              </div>
            )}
          </div>

          {/* Save/Cancel Buttons */}
          {isEditing && (
            <div className="flex items-center justify-end space-x-3 pt-4 border-t border-slate-700/50">
              <Button variant="outline" onClick={handleCancel}>
                Cancel
              </Button>
              <Button variant="sacred" onClick={handleSave}>
                <Save className="h-4 w-4 mr-1" />
                Save Changes
              </Button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}