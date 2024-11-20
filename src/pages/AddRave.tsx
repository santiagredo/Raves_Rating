import { useEffect, useState } from "react";
import "./AddRave.css";
import getAllLocations from "../lib/getAllLocations";
import { Location } from "../types/location";
import { Rating } from "../types/rating";
import getAllRatings from "../lib/getAllRatings";
import { capitalizeWords } from "../utils/capitalizeWords";
import postRaveDetail from "../lib/postRaveDetail";
import { useNavigate } from "react-router-dom";

export default function AddRave() {
    const navigate = useNavigate();

    const [name, setName] = useState("");

    const [day, setDay] = useState("");
    const [month, setMonth] = useState("");
    const [year, setYear] = useState("");

    const [locations, setLocations] = useState<Location[]>([]);
    const [ratings, setAllRatings] = useState<Rating[]>([]);

    const [locationId, setLocationId] = useState<number>(0);
    const [ratingId, setRatingId] = useState<number>(0);

    // const [isValid, setIsValid] = useState(false);

    const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;

        setName(value);
    };

    const handleDayChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        if (/^\d*$/.test(value)) {
            setDay(value);
        }
    };

    const handleMonthChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        if (/^\d*$/.test(value)) {
            setMonth(value);
        }
    };

    const handleYearChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        if (/^\d*$/.test(value)) {
            setYear(value);
        }
    };

    const handleLocationChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        setLocationId(Number(e.target.value));
    };

    const handleRatingChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        setRatingId(Number(e.target.value));
    };

    // const handleIsValid = () => {
    //     const numbers = [day, month, year, locationId, ratingId].every(
    //         (item) => item
    //     );

    //     if (name == "" || !numbers) {
    //         setIsValid(false);
    //     }
    // };

    useEffect(() => {
        const fetchLocations = async () => {
            try {
                const fetchedLocations = await getAllLocations();
                setLocations(fetchedLocations);
            } catch (error) {
                console.log(error);
            }
        };

        const fetchRatings = async () => {
            try {
                const fetchedRatings = await getAllRatings();
                setAllRatings(fetchedRatings);
            } catch (error) {
                console.log(error);
            }
        };

        fetchLocations();
        fetchRatings();
    }, []);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        const numbers = [day, month, year, locationId, ratingId].every(
            (item) => item
        );

        if (name == "" || !numbers) {
            return;
        }

        let response = await postRaveDetail(
            name,
            locationId,
            ratingId,
            `${year}-${month}-${day}`
        );

        navigate(`/rave/${response}`);
    };

    return (
        <div className="container">
            <h1>Create a New Rave</h1>

            <form onSubmit={handleSubmit} method="post" className="form-rave">
                <div>
                    <label htmlFor="name">Name:</label>
                    <input
                        type="text"
                        id="name"
                        name="name"
                        required
                        onChange={handleNameChange}
                    />
                </div>

                <div className="div-date">
                    <label htmlFor="date">Date:</label>

                    <div className="div-date-container">
                        <input
                            required
                            type="tel"
                            className="input-date"
                            name="day"
                            value={day}
                            placeholder="DD"
                            onInput={handleDayChange}
                            maxLength={2}
                        />

                        <label>-</label>

                        <input
                            required
                            type="tel"
                            className="input-date"
                            name="month"
                            value={month}
                            placeholder="MM"
                            onInput={handleMonthChange}
                            maxLength={2}
                        />

                        <label>-</label>

                        <input
                            required
                            type="tel"
                            className="input-date-year"
                            name="year"
                            value={year}
                            placeholder="YYYY"
                            onInput={handleYearChange}
                            minLength={4}
                            maxLength={4}
                        />
                    </div>
                </div>

                <div>
                    <label htmlFor="location">Location:</label>

                    <select
                        id="location"
                        name="location"
                        required
                        value={locationId}
                        onChange={handleLocationChange}
                    >
                        <option key={"0"} value={0}>
                            Select A Location
                        </option>

                        {locations.map((location: Location) => {
                            return (
                                <option key={location.id} value={location.id}>
                                    {capitalizeWords(location.name)}
                                </option>
                            );
                        })}
                    </select>
                </div>

                <div>
                    <label htmlFor="rating">Rating:</label>

                    <select
                        id="rating"
                        name="rating"
                        required
                        value={ratingId}
                        onChange={handleRatingChange}
                    >
                        <option key={"0"} value={0}>
                            Select A Rating
                        </option>

                        {ratings.map((rating) => {
                            return (
                                <option key={rating.id} value={rating.id}>
                                    {capitalizeWords(rating.name)}
                                </option>
                            );
                        })}
                    </select>
                </div>

                <button type="submit" className={`button-create`}>
                    Create Rave
                </button>
            </form>
        </div>
    );
}
