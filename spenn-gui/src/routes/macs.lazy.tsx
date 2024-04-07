import { createLazyFileRoute } from '@tanstack/react-router'
import { useState } from 'react';
import { Mac } from '../types/mac';
import { LoadingComponent } from '../components/loading';

export const Route = createLazyFileRoute('/macs')({
  component: () => Page(),
})

function Page() {
  const [macs, setMacs] = useState<Mac[]>();

  if (macs == undefined) return <LoadingComponent />

  return (
    <div className="p-2">
      <h3>Hello /macs!</h3>
    </div>
  );
}
