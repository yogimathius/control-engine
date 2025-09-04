'use client';

import { useState } from 'react';
import { useAuth } from '@/lib/auth-context';
import { Practitioner } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { RitualInterface } from '@/components/ritual/ritual-interface';
import { StateVisualization } from '@/components/state/state-visualization';
import { RitualLibrary } from '@/components/ritual/ritual-library';
import { ProfileSection } from '@/components/profile/profile-section';
import { 
  Sparkles, 
  Circle, 
  BookOpen, 
  User, 
  LogOut,
  Menu,
  X
} from 'lucide-react';

interface DashboardProps {
  user: Practitioner;
}

type DashboardView = 'ritual' | 'state' | 'library' | 'profile';

export function Dashboard({ user }: DashboardProps) {
  const [currentView, setCurrentView] = useState<DashboardView>('ritual');
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const { logout } = useAuth();

  const views = [
    { id: 'ritual' as DashboardView, label: 'Sacred Ritual', icon: Sparkles },
    { id: 'state' as DashboardView, label: 'Archetypal State', icon: Circle },
    { id: 'library' as DashboardView, label: 'Ritual Library', icon: BookOpen },
    { id: 'profile' as DashboardView, label: 'Sacred Profile', icon: User },
  ];

  const renderCurrentView = () => {
    switch (currentView) {
      case 'ritual':
        return <RitualInterface user={user} />;
      case 'state':
        return <StateVisualization user={user} />;
      case 'library':
        return <RitualLibrary user={user} />;
      case 'profile':
        return <ProfileSection user={user} />;
      default:
        return <RitualInterface user={user} />;
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
      {/* Header */}
      <header className="bg-slate-900/50 backdrop-blur-sm border-b border-slate-700/50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center space-x-3">
              <Sparkles className="h-8 w-8 text-purple-400" />
              <div>
                <h1 className="text-xl font-bold text-white">
                  Codex Control Engine
                </h1>
                <p className="text-xs text-slate-400">
                  Sacred Digital Transformation
                </p>
              </div>
            </div>

            {/* Desktop Navigation */}
            <nav className="hidden md:flex space-x-1">
              {views.map((view) => {
                const Icon = view.icon;
                return (
                  <button
                    key={view.id}
                    onClick={() => setCurrentView(view.id)}
                    className={`flex items-center space-x-2 px-4 py-2 rounded-md text-sm font-medium transition-all ${
                      currentView === view.id
                        ? 'bg-purple-600 text-white shadow-lg'
                        : 'text-slate-300 hover:bg-slate-800 hover:text-white'
                    }`}
                  >
                    <Icon className="h-4 w-4" />
                    <span>{view.label}</span>
                  </button>
                );
              })}
            </nav>

            <div className="flex items-center space-x-3">
              {/* User Info */}
              <div className="hidden sm:block text-right">
                <p className="text-sm font-medium text-white">
                  {user.spiritual_name || user.email.split('@')[0]}
                </p>
                <p className="text-xs text-slate-400">
                  {user.sacred_path || 'Practitioner'}
                </p>
              </div>

              {/* Mobile Menu Button */}
              <button
                onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
                className="md:hidden p-2 rounded-md text-slate-400 hover:text-white hover:bg-slate-800"
              >
                {isMobileMenuOpen ? (
                  <X className="h-6 w-6" />
                ) : (
                  <Menu className="h-6 w-6" />
                )}
              </button>

              {/* Logout Button */}
              <Button
                variant="ghost"
                size="sm"
                onClick={logout}
                className="hidden sm:flex text-slate-400 hover:text-white"
              >
                <LogOut className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>

        {/* Mobile Navigation */}
        {isMobileMenuOpen && (
          <div className="md:hidden border-t border-slate-700">
            <div className="px-4 py-2 space-y-1">
              {views.map((view) => {
                const Icon = view.icon;
                return (
                  <button
                    key={view.id}
                    onClick={() => {
                      setCurrentView(view.id);
                      setIsMobileMenuOpen(false);
                    }}
                    className={`flex items-center space-x-2 w-full px-3 py-2 rounded-md text-sm font-medium transition-all ${
                      currentView === view.id
                        ? 'bg-purple-600 text-white'
                        : 'text-slate-300 hover:bg-slate-800 hover:text-white'
                    }`}
                  >
                    <Icon className="h-4 w-4" />
                    <span>{view.label}</span>
                  </button>
                );
              })}
              <button
                onClick={logout}
                className="flex items-center space-x-2 w-full px-3 py-2 rounded-md text-sm font-medium text-slate-300 hover:bg-slate-800 hover:text-white"
              >
                <LogOut className="h-4 w-4" />
                <span>Sign Out</span>
              </button>
            </div>
          </div>
        )}
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {renderCurrentView()}
      </main>
    </div>
  );
}