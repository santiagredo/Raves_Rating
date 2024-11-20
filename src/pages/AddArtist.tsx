import "./AddArtist.css";

export default function AddArtist() {
    return (
        <div className="form-container">
            <h1 className="page-title">Create New Artist</h1>
            <form id="createArtistForm" className="artist-form">
                <div>
                    <label className="form-label">Artist Name:</label>
                    <input
                        type="text"
                        id="artistName"
                        name="artistName"
                        className="form-input"
                        required
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
