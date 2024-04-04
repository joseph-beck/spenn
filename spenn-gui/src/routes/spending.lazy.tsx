import { createLazyFileRoute } from '@tanstack/react-router'
import { Expense } from '../types/expense';
import { LoadingComponent } from '../components/loading';
import { useState } from 'react';

export const Route = createLazyFileRoute('/spending')({
  component: () => Spending(),
})

function Spending() {
  const [expenses, setExpenses] = useState<Expense[]>();

  if (expenses == undefined) return <LoadingComponent />

  return (
    <div className="p-2">
      <h3>Hello /spending!</h3>
    </div>
  );
}
