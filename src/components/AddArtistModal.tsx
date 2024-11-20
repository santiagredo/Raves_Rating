import { useEffect, useState } from "react";
import "./AddArtistModal.css";
import { Artist } from "../types/artist";
import getAllArtists from "../lib/getAllArtists";
import { capitalizeWords } from "../utils/capitalizeWords";
import ReactDOM from "react-dom";
import postArtist from "../lib/postArtist";
import postRaveArtistRelation from "../lib/postRaveArtistRelation";
import { useParams } from "react-router-dom";

interface AddArtistProps {
    isOpen: boolean;
    onCancel: () => void;
}

export default function AddArtistModal({ isOpen, onCancel }: AddArtistProps) {
    if (!isOpen) return null; // Do not render if modal is not open
    const { id } = useParams<{ id: string }>();

    const [allArtists, setAllArtists] = useState<Artist[]>([]);

    const [isCreateArtistOpen, setIsCreateArtistOpen] = useState(false);

    const [artistId, setArtistId] = useState(0);
    const [artistName, setArtistName] = useState("");
    const [artistCountry, setArtistCountry] = useState("");

    const handleArtistChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        setArtistId(Number(e.target.value));
    };

    const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;

        setArtistName(value);
    };

    const handleCountryChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;

        setArtistCountry(value);
    };

    const onConfirm = async () => {
        if (isCreateArtistOpen && (artistName == "" || artistCountry == "")) {
            return;
        }

        let newArtistId = artistId;

        if (isCreateArtistOpen) {
            const response = (await postArtist(artistName, artistCountry)).id;
            newArtistId = response;
        }

        await postRaveArtistRelation(Number(id!), newArtistId);

        isOpen = false;
        window.location.reload();
    };

    useEffect(() => {
        const fetchAllArtists = async () => {
            try {
                const fetchedArtists = await getAllArtists();
                setAllArtists(fetchedArtists);
            } catch (error) {
                console.log(error);
            }
        };

        fetchAllArtists();
    }, []);

    return ReactDOM.createPortal(
        <div
            id="artistModal"
            className="add-artist-modal-overlay"
            onClick={onCancel}
        >
            <div
                className="add-artist-modal"
                onClick={(e) => e.stopPropagation()}
            >
                <div className="add-artist-modal-header">
                    <h2 className="add-artist-modal-title">
                        Select or Create Artist
                    </h2>
                </div>

                <div className="add-artist-modal-body">
                    <div className="add-artist-modal-tabs">
                        <button
                            className={`add-artist-modal-tab ${
                                !isCreateArtistOpen
                                    ? "add-artist-modal-tab-active"
                                    : ""
                            }`}
                            onClick={() => setIsCreateArtistOpen(false)}
                        >
                            Select Existing
                        </button>

                        <button
                            className={`add-artist-modal-tab ${
                                isCreateArtistOpen
                                    ? "add-artist-modal-tab-active"
                                    : ""
                            }`}
                            onClick={() => setIsCreateArtistOpen(true)}
                        >
                            Create New
                        </button>
                    </div>

                    {!isCreateArtistOpen && (
                        <div
                            id="selectTab"
                            className="add-artist-modal-tab-content add-artist-modal-tab-content-active"
                        >
                            <label className="add-artist-modal-label">
                                Choose an artist:
                            </label>

                            <select
                                id="existingArtist"
                                className="add-artist-modal-select"
                                onChange={handleArtistChange}
                            >
                                <option value={0}>Select An Artist</option>
                                {allArtists.map((artist) => (
                                    <option key={artist.id} value={artist.id}>
                                        {capitalizeWords(artist.name)}
                                    </option>
                                ))}
                            </select>
                        </div>
                    )}

                    {isCreateArtistOpen && (
                        <div
                            id="createTab"
                            className="add-artist-modal-tab-content"
                        >
                            <label className="add-artist-modal-label">
                                Artist Name:
                            </label>
                            <input
                                type="text"
                                id="newArtistName"
                                placeholder="Enter artist name"
                                className="add-artist-modal-input"
                                onChange={handleNameChange}
                            />
                            <label className="add-artist-modal-label">
                                Country:
                            </label>
                            <input
                                type="text"
                                id="newArtistCountry"
                                placeholder="Enter artist country"
                                className="add-artist-modal-input"
                                onChange={handleCountryChange}
                            />
                        </div>
                    )}
                </div>

                <div className="add-artist-modal-footer">
                    <button
                        className="add-artist-modal-button add-artist-modal-button-confirm"
                        onClick={onConfirm}
                    >
                        Confirm
                    </button>
                    <button
                        className="add-artist-modal-button add-artist-modal-button-cancel"
                        onClick={onCancel}
                    >
                        Cancel
                    </button>
                </div>
            </div>
        </div>,
        document.getElementById("modal-root")!
    );
}
