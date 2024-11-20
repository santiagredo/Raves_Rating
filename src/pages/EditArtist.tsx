import { useParams } from "react-router-dom";
import "./AddArtist.css";
import { Artist } from "../types/artist";
import { useEffect, useState } from "react";
import { getArtistDetail } from "../lib/getArtistDetail";

export default function EditArtist() {
    const [artist, setArtist] = useState<Artist>();
    const { id } = useParams<{ id: string }>();

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
        <div className="form-container">
            <h1 className="page-title">Edit Artist</h1>
            <form id="createArtistForm" className="artist-form">
                <div>
                    <label className="form-label">Artist Name:</label>
                    <input
                        type="text"
                        id="artistName"
                        name="artistName"
                        className="form-input"
                        required
                        value={artist?.name}
                    />
                    <span id="nameError" className="error-message"></span>
                </div>
                <div>
                    <label className="form-label">Country:</label>
                    <input
                        type="text"
                        id="artistCountry"
                        name="artistCountry"
                        className="form-input"
                        required
                        value={artist?.country}
                    />
                    <span id="countryError" className="error-message"></span>
                </div>
                <button type="submit" className="submit-button">
                    Create Artist
                </button>
            </form>
        </div>
    );
}
