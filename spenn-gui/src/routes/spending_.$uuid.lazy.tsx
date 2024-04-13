import { createFileRoute } from '@tanstack/react-router'
import axios from 'axios';
import { useEffect, useState } from 'react';
import { Expense } from '../types/expense';
import { LoadingComponent } from '../components/loading';
import { BreadcrumbsComponent } from '../components/breadcrumbs';

export const Route = createFileRoute('/spending/$uuid')({
  component: () => Page(),
})

function Page() {
  const { uuid } = Route.useParams();
  const [expense, setExpense] = useState<Expense>();

  const get = () => {
    axios.get(`http://localhost:8080/api/v1/expenses/${uuid}`)
      .then((response) => {
        setExpense(response.data);
        console.log(response.data);
      })
      .catch((error) => {
        console.error(error);
      });
  }

  useEffect(() => { get() }, [])

  if (expense == undefined) return <LoadingComponent />

  return (
    <div>
      <h3>Hello /spending/${uuid}!</h3>

      <BreadcrumbsComponent breadcrumbs={[{ name: "home", href: "/" }, { name: "spending", href: "/spending" }]} current={{ name: uuid, href: `/spending/${uuid}` }} />

      <>{JSON.stringify(expense)}</>
    </div>
  );
}
