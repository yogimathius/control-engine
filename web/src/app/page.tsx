'use client';

import { useAuth } from '@/lib/auth-context';
import { AuthPage } from '@/components/auth/auth-page';
import { Dashboard } from '@/components/dashboard/dashboard';
import { LoadingSpinner } from '@/components/ui/loading-spinner';

export default function Home() {
  const { user, isLoading, isAuthenticated } = useAuth();

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <LoadingSpinner size="large" />
      </div>
    );
  }

  if (!isAuthenticated) {
    return <AuthPage />;
  }

  return <Dashboard user={user!} />;
}
