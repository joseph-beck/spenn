import { Link } from "@tanstack/react-router";
import { Expense } from "../types/expense";

export const ExpenseComponent = (props: {
  expense: Expense
}): JSX.Element => {
  return (
    <>
      <Link to={`/spending/${props.expense.uuid}` as string}>
        <p>{JSON.stringify(props.expense)}</p>
      </Link>
    </>
  );
}

export const ExpenseFormComponent = (props: {
  onSubmit: (expense: Expense) => void,
}): JSX.Element => {
  const handler = (e: any) => {
    e.preventDefault();
    props.onSubmit({ uuid: "123", name: "name", expenseType: 0, amount: 0, description: "description" });
  }

  return (
    <>
      <form className="" onSubmit={handler}>
        <label className="">
          name
          <input className="mx-2" type="text" name="name" />
        </label>
        <label className="">
          amount
          <input className="mx-2" type="number" name="amount" />
        </label>

        <button type="submit">submit</button>
      </form>
    </>
  );
};
