'use client';

import { useState } from 'react';
import { useAuth } from '@/lib/auth-context';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { LoadingSpinner } from '@/components/ui/loading-spinner';
import { toast } from 'react-hot-toast';
import { Eye, EyeOff, Sparkles } from 'lucide-react';

export function AuthPage() {
  const [isLogin, setIsLogin] = useState(true);
  const [showPassword, setShowPassword] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    spiritualName: '',
  });

  const { login, register } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      if (isLogin) {
        await login(formData.email, formData.password);
        toast.success('Welcome back, sacred practitioner');
      } else {
        await register(formData.email, formData.password, formData.spiritualName);
        toast.success('Sacred journey begins. Welcome to the Codex.');
      }
    } catch {
      toast.error(isLogin ? 'Login failed' : 'Registration failed');
    } finally {
      setIsLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData(prev => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  return (
    <div className="min-h-screen flex items-center justify-center px-4">
      <div className="max-w-md w-full space-y-8">
        {/* Header */}
        <div className="text-center">
          <div className="flex items-center justify-center mb-4">
            <Sparkles className="h-12 w-12 text-purple-400" />
          </div>
          <h1 className="text-4xl font-bold text-white mb-2">
            Codex Control Engine
          </h1>
          <p className="text-slate-400 text-lg">
            Sacred Digital Transformation
          </p>
          <p className="text-slate-500 text-sm mt-2">
            A platform for archetypal integration and spiritual evolution
          </p>
        </div>

        {/* Auth Form */}
        <div className="bg-slate-800/30 backdrop-blur-sm rounded-lg p-8 shadow-2xl border border-slate-700/50">
          <div className="text-center mb-6">
            <h2 className="text-2xl font-semibold text-white">
              {isLogin ? 'Enter Sacred Space' : 'Begin Your Journey'}
            </h2>
            <p className="text-slate-400 mt-1">
              {isLogin ? 'Welcome back, practitioner' : 'Create your sacred profile'}
            </p>
          </div>

          <form onSubmit={handleSubmit} className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-slate-300 mb-2">
                Email Address
              </label>
              <Input
                name="email"
                type="email"
                value={formData.email}
                onChange={handleInputChange}
                placeholder="Enter your email"
                required
              />
            </div>

            {!isLogin && (
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Spiritual Name (Optional)
                </label>
                <Input
                  name="spiritualName"
                  type="text"
                  value={formData.spiritualName}
                  onChange={handleInputChange}
                  placeholder="Your chosen sacred name"
                />
                <p className="text-xs text-slate-500 mt-1">
                  This will be your identity in sacred spaces
                </p>
              </div>
            )}

            <div>
              <label className="block text-sm font-medium text-slate-300 mb-2">
                Password
              </label>
              <div className="relative">
                <Input
                  name="password"
                  type={showPassword ? 'text' : 'password'}
                  value={formData.password}
                  onChange={handleInputChange}
                  placeholder="Enter your password"
                  required
                  className="pr-10"
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute inset-y-0 right-0 pr-3 flex items-center text-slate-400 hover:text-slate-300"
                >
                  {showPassword ? (
                    <EyeOff className="h-4 w-4" />
                  ) : (
                    <Eye className="h-4 w-4" />
                  )}
                </button>
              </div>
            </div>

            <Button
              type="submit"
              variant="sacred"
              className="w-full"
              disabled={isLoading}
            >
              {isLoading ? (
                <LoadingSpinner size="small" />
              ) : (
                isLogin ? 'Enter Sacred Space' : 'Begin Sacred Journey'
              )}
            </Button>
          </form>

          <div className="mt-6 text-center">
            <button
              onClick={() => setIsLogin(!isLogin)}
              className="text-purple-400 hover:text-purple-300 text-sm transition-colors"
            >
              {isLogin 
                ? "New to the sacred realm? Create an account"
                : "Already have an account? Sign in"
              }
            </button>
          </div>
        </div>

        {/* Sacred Quote */}
        <div className="text-center">
          <blockquote className="text-slate-400 italic text-sm">
            &ldquo;The privilege of a lifetime is to become who you truly are.&rdquo;
          </blockquote>
          <cite className="text-slate-500 text-xs block mt-1">- Carl Jung</cite>
        </div>
      </div>
    </div>
  );
}