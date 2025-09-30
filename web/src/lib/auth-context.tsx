'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { authAPI, Practitioner } from './api';

interface AuthContextType {
  user: Practitioner | null;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string, spiritualName?: string) => Promise<void>;
  logout: () => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<Practitioner | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const isAuthenticated = !!user;

  // Check for existing token on mount
  useEffect(() => {
    const checkAuth = async () => {
      const token = localStorage.getItem('sacred_token');
      if (token) {
        try {
          const profile = await authAPI.getProfile();
          setUser(profile);
        } catch (_error) {
          // Token is invalid, remove it
          localStorage.removeItem('sacred_token');
        }
      }
      setIsLoading(false);
    };

    checkAuth();
  }, []);

  const login = async (email: string, password: string) => {
    try {
      await authAPI.login(email, password);
      const profile = await authAPI.getProfile();
      setUser(profile);
    } catch (_error) {
      throw new Error('Login failed');
    }
  };

  const register = async (email: string, password: string, spiritualName?: string) => {
    try {
      await authAPI.register(email, password, spiritualName);
      // Auto-login after registration
      await login(email, password);
    } catch (_error) {
      throw new Error('Registration failed');
    }
  };

  const logout = () => {
    authAPI.logout();
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{
      user,
      isLoading,
      login,
      register,
      logout,
      isAuthenticated,
    }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}