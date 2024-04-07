import { createLazyFileRoute } from '@tanstack/react-router'
import { useState } from 'react';
import { Analytics } from '../types/analytics';
import { LoadingComponent } from '../components/loading';

export const Route = createLazyFileRoute('/analytics')({
  component: () => Page(),
})

function Page() {
  const [analytics, setAnalytics] = useState<Analytics>();

  if (analytics == undefined) return <LoadingComponent />

  return (
    <div>
      <h3>Hello /analytics!</h3>
    </div>
  );
}
