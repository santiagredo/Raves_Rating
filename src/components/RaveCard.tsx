import { Link } from "react-router-dom";
import { RaveOverview } from "../types/raveOverview";
import "./RaveCard.css";
import { capitalizeWords } from "../utils/capitalizeWords";

type CardProps = {
    rave: RaveOverview;
};

export const RaveCard: React.FC<CardProps> = ({ rave }) => {
    return (
        <Link key={rave.id} to={`/rave/${rave.id}`} className="card">
            <h3>{capitalizeWords(rave.name)}</h3>
            <p>Date: {rave.date}</p>
            <p>Location: {capitalizeWords(rave.location)}</p>
            <p>Rating: {capitalizeWords(rave.rating)}</p>
        </Link>
    );
};
