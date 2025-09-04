import axios from 'axios';

// API base configuration
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

export const api = axios.create({
  baseURL: `${API_BASE_URL}/api`,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth token to requests if available
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('sacred_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Types for API responses
export interface Practitioner {
  id: string;
  email: string;
  spiritual_name?: string;
  archetypal_preferences: Record<string, unknown>;
  energy_alignments: Record<string, unknown>;
  privacy_level: string;
  sacred_path?: string;
  created_at: string;
}

export interface ArchetypalState {
  id: string;
  practitioner_id: string;
  state_data: Record<string, unknown>;
  archetypes: Record<string, number>;
  energies: Record<string, number>;
  integrations: unknown[];
  symbols: unknown[];
  transformations: unknown[];
  state_hash: string;
  created_at: string;
}

export interface SacredRitual {
  id: string;
  name: string;
  description: string;
  intent: string;
  tradition: string;
  difficulty_level: string;
  required_archetypes: string[];
  energy_requirements: Record<string, number>;
  wasm_module_data?: Uint8Array;
  wasm_module_hash?: string;
  author_id?: string;
  usage_count: number;
  effectiveness_rating: number;
  rating_count: number;
  is_public: boolean;
  tags: string[];
  created_at: string;
}

export interface RitualSession {
  id: string;
  practitioner_id: string;
  ritual_id: string;
  execution_duration_ms?: number;
  transformation_intensity?: number;
  subjective_experience?: string;
  ai_interpretation?: string;
  integration_notes?: string;
  effectiveness_rating?: number;
  ritual_parameters: Record<string, unknown>;
  intention?: string;
  created_at: string;
}

// Authentication API
export const authAPI = {
  register: async (email: string, password: string, spiritualName?: string) => {
    const response = await api.post('/users/register', {
      email,
      password,
      spiritual_name: spiritualName,
    });
    return response.data;
  },

  login: async (email: string, password: string) => {
    const response = await api.post('/users/login', { email, password });
    if (response.data.token) {
      localStorage.setItem('sacred_token', response.data.token);
    }
    return response.data;
  },

  logout: () => {
    localStorage.removeItem('sacred_token');
  },

  getProfile: async (): Promise<Practitioner> => {
    const response = await api.get('/users/profile');
    return response.data;
  },
};

// Rituals API
export const ritualsAPI = {
  getCatalog: async (): Promise<SacredRitual[]> => {
    const response = await api.get('/rituals/catalog');
    return response.data;
  },

  getRitualDetails: async (id: string): Promise<SacredRitual> => {
    const response = await api.get(`/rituals/${id}`);
    return response.data;
  },

  executeRitual: async (ritualId: string, parameters: Record<string, unknown>, intention?: string): Promise<RitualSession> => {
    const response = await api.post('/rituals/execute', {
      ritual_id: ritualId,
      parameters,
      intention,
    });
    return response.data;
  },

  uploadRitual: async (ritualData: Omit<SacredRitual, 'id' | 'created_at' | 'usage_count' | 'effectiveness_rating' | 'rating_count'>) => {
    const response = await api.post('/rituals/upload', ritualData);
    return response.data;
  },
};

// State API
export const stateAPI = {
  getCurrentState: async (): Promise<ArchetypalState> => {
    const response = await api.get('/state/current');
    return response.data;
  },

  getStateHistory: async (): Promise<ArchetypalState[]> => {
    const response = await api.get('/state/history');
    return response.data;
  },

  transformState: async (transformation: Record<string, unknown>) => {
    const response = await api.post('/state/transform', transformation);
    return response.data;
  },

  requestReflection: async (sessionId?: string): Promise<{ reflection: string }> => {
    const response = await api.post('/state/reflection', { session_id: sessionId });
    return response.data;
  },
};