import { invoke } from "@tauri-apps/api/core";
import { getExpenseType } from "../types/expense";

export default async function postExpense(
    name: string,
    price: number,
    rave_id: number
): Promise<number> {
    let expense = getExpenseType();

    expense.name = name;
    expense.price = price;
    expense.rave_id = rave_id;

    try {
        const response = await invoke<number>("insert_expenses", {
            expenses_collection: [expense],
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
