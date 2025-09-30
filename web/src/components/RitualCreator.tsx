'use client';

import React, { useState } from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';
import { Plus, X, Upload, Code, Sparkles } from 'lucide-react';
import { useAuth } from '@/hooks/useAuth';

interface CreatedRitual {
  id: string;
  name: string;
  description: string;
  intent: string;
  required_archetypes: string[];
  energy_requirements: Record<string, number>;
  wasm_module_path?: string;
  parameters: Record<string, unknown>;
}

interface RitualCreatorProps {
  onRitualCreated?: (ritual: CreatedRitual) => void;
}

interface ArchetypeRequirement {
  name: string;
  minimumActivation: number;
}

interface EnergyRequirement {
  type: string;
  amount: number;
}

const AVAILABLE_ARCHETYPES = [
  'Sage', 'Creator', 'Shadow', 'Light', 'Warrior', 'Lover', 'Ruler', 'Magician',
  'Innocent', 'Explorer', 'Hero', 'Caregiver', 'Rebel', 'Jester'
];

const AVAILABLE_ENERGIES = [
  'Fire', 'Water', 'Earth', 'Air', 'Void', 'Light', 'Shadow', 'Spirit'
];

const TRADITIONS = [
  'universal', 'jungian', 'shamanic', 'hermetic', 'alchemical', 'tantric', 
  'zen', 'kabbalastic', 'celtic', 'computational_alchemy'
];

const DIFFICULTY_LEVELS = [
  'beginner', 'intermediate', 'advanced', 'master'
];

export function RitualCreator({ onRitualCreated }: RitualCreatorProps) {
  const { user } = useAuth();
  const [isCreating, setIsCreating] = useState(false);
  const [currentStep, setCurrentStep] = useState(1);
  
  // Form state
  const [ritualData, setRitualData] = useState({
    name: '',
    description: '',
    intent: '',
    tradition: 'universal',
    difficulty_level: 'beginner',
    is_public: false,
    tags: [] as string[],
    required_archetypes: [] as ArchetypeRequirement[],
    energy_requirements: [] as EnergyRequirement[],
    wasm_code: '',
    module_language: 'rust',
  });

  const [newTag, setNewTag] = useState('');
  const [wasmFile, setWasmFile] = useState<File | null>(null);

  const handleInputChange = (field: string, value: string | number) => {
    setRitualData(prev => ({
      ...prev,
      [field]: value
    }));
  };

  const addArchetypeRequirement = () => {
    if (ritualData.required_archetypes.length < 5) {
      setRitualData(prev => ({
        ...prev,
        required_archetypes: [...prev.required_archetypes, { name: '', minimumActivation: 0.3 }]
      }));
    }
  };

  const updateArchetypeRequirement = (index: number, field: string, value: string | number) => {
    setRitualData(prev => ({
      ...prev,
      required_archetypes: prev.required_archetypes.map((req, i) => 
        i === index ? { ...req, [field]: value } : req
      )
    }));
  };

  const removeArchetypeRequirement = (index: number) => {
    setRitualData(prev => ({
      ...prev,
      required_archetypes: prev.required_archetypes.filter((_, i) => i !== index)
    }));
  };

  const addEnergyRequirement = () => {
    if (ritualData.energy_requirements.length < 8) {
      setRitualData(prev => ({
        ...prev,
        energy_requirements: [...prev.energy_requirements, { type: '', amount: 0.2 }]
      }));
    }
  };

  const updateEnergyRequirement = (index: number, field: string, value: string | number) => {
    setRitualData(prev => ({
      ...prev,
      energy_requirements: prev.energy_requirements.map((req, i) => 
        i === index ? { ...req, [field]: value } : req
      )
    }));
  };

  const removeEnergyRequirement = (index: number) => {
    setRitualData(prev => ({
      ...prev,
      energy_requirements: prev.energy_requirements.filter((_, i) => i !== index)
    }));
  };

  const _addTag = () => {
    if (newTag.trim() && !ritualData.tags.includes(newTag.trim())) {
      setRitualData(prev => ({
        ...prev,
        tags: [...prev.tags, newTag.trim()]
      }));
      setNewTag('');
    }
  };

  const _removeTag = (tagToRemove: string) => {
    setRitualData(prev => ({
      ...prev,
      tags: prev.tags.filter(tag => tag !== tagToRemove)
    }));
  };

  const handleWasmFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file && file.name.endsWith('.wasm')) {
      setWasmFile(file);
    }
  };

  const createRitual = async () => {
    if (!user?.token) return;
    
    setIsCreating(true);
    
    try {
      // Prepare ritual data
      const ritualPayload = {
        name: ritualData.name,
        description: ritualData.description,
        intent: ritualData.intent,
        tradition: ritualData.tradition,
        difficulty_level: ritualData.difficulty_level,
        required_archetypes: ritualData.required_archetypes.map(req => req.name),
        energy_requirements: Object.fromEntries(
          ritualData.energy_requirements.map(req => [req.type, req.amount])
        ),
        is_public: ritualData.is_public,
        module_language: ritualData.module_language,
        wasm_module: wasmFile ? await fileToBase64(wasmFile) : null,
      };

      const response = await fetch('/api/rituals/upload', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${user.token}`,
        },
        body: JSON.stringify(ritualPayload),
      });

      if (response.ok) {
        const result = await response.json();
        onRitualCreated?.(result.data);
        
        // Reset form
        setRitualData({
          name: '',
          description: '',
          intent: '',
          tradition: 'universal',
          difficulty_level: 'beginner',
          is_public: false,
          tags: [],
          required_archetypes: [],
          energy_requirements: [],
          wasm_code: '',
          module_language: 'rust',
        });
        setCurrentStep(1);
        setWasmFile(null);
      } else {
        console.error('Failed to create ritual');
      }
    } catch (error) {
      console.error('Error creating ritual:', error);
    } finally {
      setIsCreating(false);
    }
  };

  const fileToBase64 = (file: File): Promise<string> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.readAsArrayBuffer(file);
      reader.onload = () => {
        const arrayBuffer = reader.result as ArrayBuffer;
        const bytes = new Uint8Array(arrayBuffer);
        const base64 = btoa(String.fromCharCode(...bytes));
        resolve(base64);
      };
      reader.onerror = error => reject(error);
    });
  };

  const nextStep = () => {
    if (currentStep < 4) {
      setCurrentStep(currentStep + 1);
    }
  };

  const prevStep = () => {
    if (currentStep > 1) {
      setCurrentStep(currentStep - 1);
    }
  };

  const canProceed = () => {
    switch (currentStep) {
      case 1:
        return ritualData.name && ritualData.description && ritualData.intent;
      case 2:
        return true; // Requirements are optional
      case 3:
        return true; // WASM is optional
      case 4:
        return true; // Review step
      default:
        return false;
    }
  };

  const renderStep = () => {
    switch (currentStep) {
      case 1:
        return (
          <div className="space-y-6">
            <div>
              <Label htmlFor="name" className="text-purple-200">Sacred Name</Label>
              <Input
                id="name"
                value={ritualData.name}
                onChange={(e) => handleInputChange('name', e.target.value)}
                placeholder="Enter the sacred name of your ritual..."
                className="bg-slate-800/50 border-purple-500/30 text-purple-100"
              />
            </div>
            
            <div>
              <Label htmlFor="description" className="text-purple-200">Description</Label>
              <Textarea
                id="description"
                value={ritualData.description}
                onChange={(e) => handleInputChange('description', e.target.value)}
                placeholder="Describe the nature and purpose of this sacred practice..."
                className="bg-slate-800/50 border-purple-500/30 text-purple-100 min-h-[100px]"
              />
            </div>
            
            <div>
              <Label htmlFor="intent" className="text-purple-200">Sacred Intent</Label>
              <Textarea
                id="intent"
                value={ritualData.intent}
                onChange={(e) => handleInputChange('intent', e.target.value)}
                placeholder="What transformation or healing does this ritual achieve?"
                className="bg-slate-800/50 border-purple-500/30 text-purple-100 min-h-[80px]"
              />
            </div>
            
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label className="text-purple-200">Tradition</Label>
                <Select value={ritualData.tradition} onValueChange={(value) => handleInputChange('tradition', value)}>
                  <SelectTrigger className="bg-slate-800/50 border-purple-500/30 text-purple-100">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {TRADITIONS.map(tradition => (
                      <SelectItem key={tradition} value={tradition}>
                        {tradition.charAt(0).toUpperCase() + tradition.slice(1).replace('_', ' ')}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div>
                <Label className="text-purple-200">Difficulty</Label>
                <Select value={ritualData.difficulty_level} onValueChange={(value) => handleInputChange('difficulty_level', value)}>
                  <SelectTrigger className="bg-slate-800/50 border-purple-500/30 text-purple-100">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {DIFFICULTY_LEVELS.map(level => (
                      <SelectItem key={level} value={level}>
                        {level.charAt(0).toUpperCase() + level.slice(1)}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
        );
      
      case 2:
        return (
          <div className="space-y-6">
            <div>
              <div className="flex items-center justify-between mb-4">
                <Label className="text-purple-200">Archetypal Requirements</Label>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={addArchetypeRequirement}
                  className="border-purple-500/30 text-purple-300 hover:bg-purple-500/10"
                >
                  <Plus className="w-4 h-4 mr-2" />
                  Add Archetype
                </Button>
              </div>
              
              <div className="space-y-3">
                {ritualData.required_archetypes.map((req, index) => (
                  <div key={index} className="flex items-center gap-3 bg-slate-800/30 p-3 rounded-lg">
                    <Select 
                      value={req.name} 
                      onValueChange={(value) => updateArchetypeRequirement(index, 'name', value)}
                    >
                      <SelectTrigger className="flex-1 bg-slate-700/50 border-purple-500/30">
                        <SelectValue placeholder="Select archetype..." />
                      </SelectTrigger>
                      <SelectContent>
                        {AVAILABLE_ARCHETYPES.map(archetype => (
                          <SelectItem key={archetype} value={archetype}>{archetype}</SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    
                    <div className="flex items-center gap-2">
                      <Label className="text-purple-300 text-sm">Min:</Label>
                      <Input
                        type="number"
                        min="0"
                        max="1"
                        step="0.1"
                        value={req.minimumActivation}
                        onChange={(e) => updateArchetypeRequirement(index, 'minimumActivation', parseFloat(e.target.value))}
                        className="w-20 bg-slate-700/50 border-purple-500/30 text-purple-100"
                      />
                    </div>
                    
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      onClick={() => removeArchetypeRequirement(index)}
                      className="text-red-400 hover:bg-red-500/10"
                    >
                      <X className="w-4 h-4" />
                    </Button>
                  </div>
                ))}
              </div>
            </div>
            
            <div>
              <div className="flex items-center justify-between mb-4">
                <Label className="text-purple-200">Energy Requirements</Label>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={addEnergyRequirement}
                  className="border-purple-500/30 text-purple-300 hover:bg-purple-500/10"
                >
                  <Plus className="w-4 h-4 mr-2" />
                  Add Energy
                </Button>
              </div>
              
              <div className="space-y-3">
                {ritualData.energy_requirements.map((req, index) => (
                  <div key={index} className="flex items-center gap-3 bg-slate-800/30 p-3 rounded-lg">
                    <Select 
                      value={req.type} 
                      onValueChange={(value) => updateEnergyRequirement(index, 'type', value)}
                    >
                      <SelectTrigger className="flex-1 bg-slate-700/50 border-purple-500/30">
                        <SelectValue placeholder="Select energy type..." />
                      </SelectTrigger>
                      <SelectContent>
                        {AVAILABLE_ENERGIES.map(energy => (
                          <SelectItem key={energy} value={energy}>{energy}</SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    
                    <div className="flex items-center gap-2">
                      <Label className="text-purple-300 text-sm">Amount:</Label>
                      <Input
                        type="number"
                        min="0"
                        max="1"
                        step="0.1"
                        value={req.amount}
                        onChange={(e) => updateEnergyRequirement(index, 'amount', parseFloat(e.target.value))}
                        className="w-20 bg-slate-700/50 border-purple-500/30 text-purple-100"
                      />
                    </div>
                    
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      onClick={() => removeEnergyRequirement(index)}
                      className="text-red-400 hover:bg-red-500/10"
                    >
                      <X className="w-4 h-4" />
                    </Button>
                  </div>
                ))}
              </div>
            </div>
          </div>
        );
      
      case 3:
        return (
          <div className="space-y-6">
            <div>
              <Label className="text-purple-200">WASM Module (Optional)</Label>
              <p className="text-purple-400 text-sm mb-4">
                Upload a compiled WebAssembly module for advanced ritual logic
              </p>
              
              <div className="border-2 border-dashed border-purple-500/30 rounded-lg p-6 text-center">
                <input
                  type="file"
                  accept=".wasm"
                  onChange={handleWasmFileUpload}
                  className="hidden"
                  id="wasm-upload"
                />
                <label
                  htmlFor="wasm-upload"
                  className="cursor-pointer flex flex-col items-center gap-3"
                >
                  <Upload className="w-8 h-8 text-purple-400" />
                  <div>
                    <p className="text-purple-200">Upload WASM Module</p>
                    <p className="text-purple-400 text-sm">Click to select a .wasm file</p>
                  </div>
                </label>
                
                {wasmFile && (
                  <div className="mt-4 p-3 bg-slate-800/50 rounded-lg">
                    <p className="text-purple-200">Selected: {wasmFile.name}</p>
                    <p className="text-purple-400 text-sm">Size: {(wasmFile.size / 1024).toFixed(1)} KB</p>
                  </div>
                )}
              </div>
            </div>
            
            <div>
              <Label className="text-purple-200">Module Language</Label>
              <Select value={ritualData.module_language} onValueChange={(value) => handleInputChange('module_language', value)}>
                <SelectTrigger className="bg-slate-800/50 border-purple-500/30 text-purple-100">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="rust">Rust</SelectItem>
                  <SelectItem value="c">C/C++</SelectItem>
                  <SelectItem value="assemblyscript">AssemblyScript</SelectItem>
                  <SelectItem value="wat">WebAssembly Text</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        );
      
      case 4:
        return (
          <div className="space-y-6">
            <h3 className="text-xl font-semibold text-purple-200">Review & Publish</h3>
            
            <Card className="bg-slate-800/50 border-purple-500/30">
              <CardHeader>
                <CardTitle className="text-purple-200">{ritualData.name}</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <p className="text-purple-300">{ritualData.description}</p>
                
                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <span className="text-purple-400">Tradition:</span>
                    <span className="ml-2 text-purple-200">{ritualData.tradition}</span>
                  </div>
                  <div>
                    <span className="text-purple-400">Difficulty:</span>
                    <span className="ml-2 text-purple-200">{ritualData.difficulty_level}</span>
                  </div>
                </div>
                
                {ritualData.required_archetypes.length > 0 && (
                  <div>
                    <span className="text-purple-400 text-sm">Required Archetypes:</span>
                    <div className="flex flex-wrap gap-2 mt-2">
                      {ritualData.required_archetypes.map((req, index) => (
                        <Badge key={index} variant="secondary" className="bg-purple-500/20 text-purple-200">
                          {req.name} ({req.minimumActivation})
                        </Badge>
                      ))}
                    </div>
                  </div>
                )}
                
                {ritualData.energy_requirements.length > 0 && (
                  <div>
                    <span className="text-purple-400 text-sm">Energy Requirements:</span>
                    <div className="flex flex-wrap gap-2 mt-2">
                      {ritualData.energy_requirements.map((req, index) => (
                        <Badge key={index} variant="outline" className="border-purple-500/30 text-purple-300">
                          {req.type} ({req.amount})
                        </Badge>
                      ))}
                    </div>
                  </div>
                )}
                
                {wasmFile && (
                  <div className="flex items-center gap-2 text-sm text-purple-300">
                    <Code className="w-4 h-4" />
                    <span>WASM Module: {wasmFile.name}</span>
                  </div>
                )}
              </CardContent>
            </Card>
            
            <div className="flex items-center gap-3">
              <input
                type="checkbox"
                id="is_public"
                checked={ritualData.is_public}
                onChange={(e) => handleInputChange('is_public', e.target.checked)}
                className="rounded border-purple-500/30"
              />
              <Label htmlFor="is_public" className="text-purple-200">
                Make this ritual public for the community
              </Label>
            </div>
          </div>
        );
      
      default:
        return null;
    }
  };

  const stepTitles = [
    'Sacred Foundation',
    'Requirements',
    'WASM Module',
    'Review & Publish'
  ];

  return (
    <Card className="max-w-4xl mx-auto bg-slate-900/50 border-purple-500/30">
      <CardHeader>
        <div className="flex items-center gap-3">
          <Sparkles className="w-6 h-6 text-purple-400" />
          <CardTitle className="text-2xl text-purple-200">Create Sacred Ritual</CardTitle>
        </div>
        
        {/* Progress indicator */}
        <div className="flex justify-between mt-6">
          {stepTitles.map((title, index) => (
            <div
              key={index}
              className={`flex items-center ${index < stepTitles.length - 1 ? 'flex-1' : ''}`}
            >
              <div className={`
                w-8 h-8 rounded-full flex items-center justify-center text-sm font-semibold
                ${currentStep > index + 1 ? 'bg-purple-500 text-white' : 
                  currentStep === index + 1 ? 'bg-purple-500/50 text-purple-100' : 
                  'bg-slate-700 text-slate-400'}
              `}>
                {index + 1}
              </div>
              <span className={`ml-2 text-sm ${
                currentStep === index + 1 ? 'text-purple-200' : 'text-slate-400'
              }`}>
                {title}
              </span>
              {index < stepTitles.length - 1 && (
                <div className={`flex-1 h-0.5 ml-4 ${
                  currentStep > index + 1 ? 'bg-purple-500' : 'bg-slate-700'
                }`} />
              )}
            </div>
          ))}
        </div>
      </CardHeader>
      
      <CardContent>
        {renderStep()}
        
        <div className="flex justify-between mt-8">
          <Button
            type="button"
            variant="outline"
            onClick={prevStep}
            disabled={currentStep === 1}
            className="border-purple-500/30 text-purple-300 hover:bg-purple-500/10"
          >
            Previous
          </Button>
          
          <div className="flex gap-3">
            {currentStep < 4 ? (
              <Button
                type="button"
                onClick={nextStep}
                disabled={!canProceed()}
                className="bg-purple-600 hover:bg-purple-700 text-white"
              >
                Next
              </Button>
            ) : (
              <Button
                type="button"
                onClick={createRitual}
                disabled={isCreating || !canProceed()}
                className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white"
              >
                {isCreating ? 'Creating...' : 'Create Ritual'}
                <Sparkles className="w-4 h-4 ml-2" />
              </Button>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}