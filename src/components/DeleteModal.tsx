import ReactDOM from "react-dom";
import "./DeleteModal.css";

interface DeleteModalProps {
    isOpen: boolean;
    onConfirm: () => void;
    onCancel: () => void;
    deleteMessage: string;
}

export default function DeleteModal({
    isOpen,
    onConfirm,
    onCancel,
    deleteMessage,
}: DeleteModalProps) {
    if (!isOpen) return null; // Do not render if modal is not open

    return ReactDOM.createPortal(
        <div className="delete-modal" onClick={onCancel}>
            <div
                className="delete-modal-content"
                onClick={(e) => e.stopPropagation()}
            >
                <h2 className="delete-modal-content-h2">Confirm Deletion</h2>
                <p>{deleteMessage}</p>

                <div className="delete-modal-buttons">
                    <button
                        className="delete-button-confirm"
                        onClick={onConfirm}
                    >
                        Confirm
                    </button>
                    <button className="delete-button-cancel" onClick={onCancel}>
                        Cancel
                    </button>
                </div>
            </div>
        </div>,

        document.getElementById("modal-root")!
    );
}
