import { invoke } from "@tauri-apps/api/core";
import { getExpenseType } from "../types/expense";

export default async function deleteExpense(id: number): Promise<number> {
    let expense = getExpenseType();
    expense.id = id;

    try {
        const response = await invoke<number>("delete_expenses", {
            expenses_collection: [expense],
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
