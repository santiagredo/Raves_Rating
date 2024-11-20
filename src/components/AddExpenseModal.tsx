import ReactDOM from "react-dom";
import "./AddExpense.css";
import { useState } from "react";
import { useParams } from "react-router-dom";
import postExpense from "../lib/postExpense";

interface AddExpenseProps {
    isOpen: boolean;
    onCancel: () => void;
}

export default function AddExpenseModal({ isOpen, onCancel }: AddExpenseProps) {
    if (!isOpen) return null;

    const { id } = useParams<{ id: string }>();

    const [name, setName] = useState("");
    const [price, setPrice] = useState(0);

    const handlePriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        if (value === "" || /^-?\d*\.?\d*$/.test(value)) {
            setPrice(value === "" ? 0 : Number(value)); // Set as empty string if deleted, otherwise convert to number
        }
    };

    const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setName(e.target.value);
    };

    const onConfirm = async () => {
        await postExpense(name, price, Number(id));
        isOpen = false;
        window.location.reload();
    };

    return ReactDOM.createPortal(
        <div
            id="expenseModal"
            className="add-expense-modal-overlay"
            onClick={onCancel}
        >
            <div
                className="add-expense-modal"
                onClick={(e) => e.stopPropagation()}
            >
                <div className="add-expense-modal-header">
                    <h2 className="add-expense-modal-title">
                        Create New Expense
                    </h2>
                </div>

                <div className="add-expense-modal-body">
                    <form id="expenseForm">
                        <label>Expense Name:</label>
                        <input
                            type="text"
                            id="expenseName"
                            name="expenseName"
                            required
                            placeholder="Enter expense name"
                            onInput={handleNameChange}
                        />

                        <label>Price:</label>
                        <input
                            type="text"
                            id="expensePrice"
                            name="expensePrice"
                            required
                            min="0"
                            step="0.01"
                            placeholder="Enter price"
                            value={price}
                            onInput={handlePriceChange}
                        />
                    </form>
                </div>
                <div className="add-expense-modal-footer">
                    <button onClick={onConfirm}>Save Expense</button>

                    <button className="secondary" onClick={onCancel}>
                        Cancel
                    </button>
                </div>
            </div>
        </div>,

        document.getElementById("modal-root")!
    );
}
