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
