import { useNavigate, useParams } from "react-router-dom";
import "./ArtistDetail.css";
import { useEffect, useState } from "react";
import { Artist } from "../types/artist";
import { getArtistDetail } from "../lib/getArtistDetail";
import { capitalizeWords } from "../utils/capitalizeWords";
import DeleteModal from "../components/DeleteModal";
import deleteArtist from "../lib/deleteArtist";

export default function ArtistDetail() {
    const navigate = useNavigate();

    const [artist, setArtist] = useState<Artist>();
    const { id } = useParams<{ id: string }>();
    const [isDeleteArtistModalOpen, setIsDeleteArtistModalOpen] =
        useState(false);

    const handleArtistDelete = async () => {
        await deleteArtist(Number(id));

        setIsDeleteArtistModalOpen(false);
        navigate("/");
    };

    useEffect(() => {
        const fetchArtistDetail = async () => {
            try {
                const fetchedArtistDetail = await getArtistDetail(Number(id));
                setArtist(fetchedArtistDetail);
            } catch (error) {
                console.log(error);
            }
        };

        fetchArtistDetail();
    }, [id]);

    return (
        <div>
            {artist ? (
                <>
                    <section>
                        <div className="artist-card">
                            <h2 className="artist-name">
                                {capitalizeWords(artist.name)}
                            </h2>
                            <p className="artist-country">
                                {capitalizeWords(artist.country)}
                            </p>
                        </div>

                        <div className="artist-options">
                            <button
                                onClick={() => navigate(`/edit-artist/${id}`)}
                            >
                                <strong>Edit Artist</strong>
                            </button>

                            <button
                                onClick={() => setIsDeleteArtistModalOpen(true)}
                            >
                                <strong>Delete Artist</strong>
                            </button>
                        </div>
                    </section>

                    <section></section>
                </>
            ) : (
                <p>Loading ...</p>
            )}

            <DeleteModal
                isOpen={isDeleteArtistModalOpen}
                onCancel={() => setIsDeleteArtistModalOpen(false)}
                onConfirm={handleArtistDelete}
                deleteMessage="Are you sure you want to delete this artist? All raves relations will be deleted as well"
            />
        </div>
    );
}
