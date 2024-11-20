import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { RaveDetail as RaveDetailType } from "../types/raveDetail";
import { getRaveDetail } from "../lib/getRaveDetail";
import "./RaveDetail.css";
import { capitalizeWords } from "../utils/capitalizeWords";
import DeleteModal from "../components/DeleteModal";
import deleteRave from "../lib/deleteRave";
import AddArtistModal from "../components/AddArtistModal";
import AddExpenseModal from "../components/AddExpenseModal";
import deleteIcon from "../assets/delete.svg";
import deleteRaveArtistRelation from "../lib/deleteRaveArtistRelation";
import deleteExpense from "../lib/deleteExpense";

export default function RaveDetail() {
    const navigate = useNavigate();
    const [isAddArtistModalOpen, setIsArtistModalOpen] = useState(false);
    const [isDeleteRaveModalOpen, setIsDeleteRaveModalOpen] = useState(false);
    const [isAddExpenseModalOpen, setIsAddExpenseModalOpen] = useState(false);
    const [isDeleteArtistModalOpen, setIsDeleteArtistModalOpen] =
        useState(false);
    const [selectedArtistId, setSelectedArtistId] = useState<number>(0);
    const [isDeleteExpenseModalOpen, setIsDeleteExpenseModalOpen] =
        useState(false);
    const [selectedExpenseId, setSelectedExpenseId] = useState<number>(0);

    const { id } = useParams<{ id: string }>();
    const [raveDetail, setRaveDetail] = useState<RaveDetailType>();

    const handleRaveDelete = async () => {
        await deleteRave(Number(id));

        setIsDeleteRaveModalOpen(false);

        navigate("/");
    };

    const handleArtistDeleteClick = (id: number) => {
        setSelectedArtistId(id);
        setIsDeleteArtistModalOpen(true);
    };

    const handleExpenseDeleteClick = (id: number) => {
        setSelectedExpenseId(id);
        setIsDeleteExpenseModalOpen(true);
    };

    const handleArtistDelete = async () => {
        await deleteRaveArtistRelation(Number(id), selectedArtistId);

        setIsDeleteArtistModalOpen(false);

        window.location.reload();
    };

    const handleExpenseDelete = async () => {
        await deleteExpense(selectedExpenseId);

        setIsDeleteExpenseModalOpen(false);

        window.location.reload();
    };

    useEffect(() => {
        const fetchRaveDetail = async () => {
            try {
                const fetchedRaveDetail = await getRaveDetail(id || "");
                setRaveDetail(fetchedRaveDetail);
            } catch (error) {
                console.log(error);
            }
        };

        fetchRaveDetail();
    }, []);

    return (
        <>
            <main>
                <article className="rave-card">
                    <header className="rave-header">
                        <h1 className="rave-name">
                            {capitalizeWords(raveDetail?.rave.name || "")}
                        </h1>
                        <span className="rave-rating">
                            {capitalizeWords(raveDetail?.rating.name || "")}
                        </span>
                    </header>

                    <div className="rave-info">
                        <p>
                            <strong>Date:</strong>{" "}
                            <time>{raveDetail?.rave.date}</time>
                        </p>

                        <p>
                            <strong>Location:</strong>{" "}
                            {capitalizeWords(raveDetail?.location.name || "")}
                        </p>

                        <p>
                            <strong>Address:</strong>{" "}
                            {capitalizeWords(
                                raveDetail?.location.address || ""
                            )}
                        </p>

                        <p>
                            <strong>Capacity:</strong>{" "}
                            {capitalizeWords(
                                raveDetail?.location.capacity || ""
                            )}
                        </p>
                    </div>
                </article>

                <section className="rave-options">
                    <button
                        className="rave-options-card"
                        onClick={() => navigate(`/edit-rave/${id}`)}
                    >
                        <strong>Edit Rave</strong>
                    </button>

                    <button
                        className="rave-options-card"
                        onClick={() => setIsArtistModalOpen(true)}
                    >
                        <strong>Add Artist</strong>
                    </button>

                    <button
                        className="rave-options-card"
                        onClick={() => setIsAddExpenseModalOpen(true)}
                    >
                        <strong>Add Expense</strong>
                    </button>

                    <button
                        className="rave-options-card"
                        onClick={() => setIsDeleteRaveModalOpen(true)}
                    >
                        <strong>Delete Rave</strong>
                    </button>
                </section>

                <section className="rave-section-artists">
                    <h2>Artists</h2>

                    <div className="rave-artists">
                        {raveDetail?.artists.map((artist) => {
                            return (
                                <article
                                    className="rave-artists-card"
                                    key={artist.id}
                                >
                                    <div>
                                        <p
                                            className="rave-artists-card-name"
                                            onClick={() =>
                                                navigate(`/artist/${artist.id}`)
                                            }
                                        >
                                            <strong>
                                                {capitalizeWords(artist.name)}
                                            </strong>
                                        </p>
                                        <p>{capitalizeWords(artist.country)}</p>
                                    </div>

                                    <button
                                        className="hover-white hover-red"
                                        onClick={() =>
                                            handleArtistDeleteClick(artist.id)
                                        }
                                    >
                                        <img
                                            alt="Delete icon"
                                            src={deleteIcon}
                                        />
                                    </button>
                                </article>
                            );
                        })}
                    </div>
                </section>

                <section className="rave-section-expenses">
                    <h2>Expenses</h2>

                    <div className="rave-expenses">
                        {raveDetail?.expenses.map((expense) => {
                            return (
                                <article
                                    className="rave-expenses-card"
                                    key={expense.id}
                                >
                                    <div>
                                        <p>
                                            <strong>
                                                {capitalizeWords(expense.name)}
                                            </strong>
                                        </p>
                                        <p>
                                            {capitalizeWords(
                                                expense.price.toString()
                                            )}
                                        </p>
                                    </div>

                                    <button
                                        className="hover-white hover-red"
                                        onClick={() =>
                                            handleExpenseDeleteClick(expense.id)
                                        }
                                    >
                                        <img
                                            alt="Delete icon"
                                            src={deleteIcon}
                                        />
                                    </button>
                                </article>
                            );
                        })}
                    </div>
                </section>
            </main>

            <DeleteModal
                isOpen={isDeleteRaveModalOpen}
                onConfirm={handleRaveDelete}
                onCancel={() => setIsDeleteRaveModalOpen(false)}
                deleteMessage="Are you sure you want to delete this rave? All associated data will be deleted."
            />

            <AddArtistModal
                isOpen={isAddArtistModalOpen}
                onCancel={() => setIsArtistModalOpen(false)}
            />

            <AddExpenseModal
                isOpen={isAddExpenseModalOpen}
                onCancel={() => setIsAddExpenseModalOpen(false)}
            />

            <DeleteModal
                isOpen={isDeleteArtistModalOpen}
                onConfirm={handleArtistDelete}
                onCancel={() => setIsDeleteArtistModalOpen(false)}
                deleteMessage="Are you sure you want to delete this artist?"
            />

            <DeleteModal
                isOpen={isDeleteExpenseModalOpen}
                onConfirm={handleExpenseDelete}
                onCancel={() => setIsDeleteExpenseModalOpen(false)}
                deleteMessage="Are you sure you want to delete this expense?"
            />
        </>
    );
}
