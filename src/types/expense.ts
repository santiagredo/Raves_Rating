export type Expense = {
    id: number;
    name: string;
    price: number;
    rave_id: number;
};

export function getExpenseType(): Expense {
    let expense = {
        id: 0,
        name: "",
        price: 0,
        rave_id: 0,
    };

    return expense;
}
