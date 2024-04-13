import { createLazyFileRoute } from '@tanstack/react-router'
import { Expense } from '../types/expense';
import { LoadingComponent } from '../components/loading';
import { useEffect, useState } from 'react';
import axios from 'axios';
import { ExpenseComponent, ExpenseFormComponent } from '../components/expense';
import { BreadcrumbsComponent } from '../components/breadcrumbs';

export const Route = createLazyFileRoute('/spending')({
  component: () => Page(),
})

function Page() {
  const [expenses, setExpenses] = useState<Expense[]>();

  const get = () => {
    axios.get("http://localhost:8080/api/v1/expenses")
      .then((response) => {
        setExpenses(response.data);
        console.log(response.data);
      })
      .catch((error) => {
        console.error(error);
      });
  }

  const post = (expense: Expense) => {
    axios.post("http://localhost:8080/api/v1/expenses", expense)
      .then((response) => {
        console.log(response.status);
      })
      .catch((error) => {
        console.error(error);
      });
  }

  useEffect(() => { get() }, [])

  if (expenses == undefined) return <LoadingComponent />

  return (
    <div className="p-2">
      <h3>Hello /spending!</h3>

      <BreadcrumbsComponent breadcrumbs={[{ name: "home", href: "/" }]} current={{ name: "spending", href: "/spending"}} />

      <div>
        {
          expenses.map((expense: Expense) => (
            <div key={expense.uuid}>
              <ExpenseComponent expense={expense} />
            </div>
          ))
        }
      </div>

      <div className="my-8">
        <ExpenseFormComponent onSubmit={(expense: Expense) => post(expense)} />
      </div>
    </div>
  );
}
