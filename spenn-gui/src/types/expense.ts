export interface Expense {
  uuid: string;
  name: string;
  expenseType: number;
  amount: number;
  description: string;
  createdAt?: string;
}
