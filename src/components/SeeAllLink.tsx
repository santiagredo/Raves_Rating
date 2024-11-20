import { Link } from "react-router-dom";
import "./SeeAllLink.css";

export const SeeAllLink = () => {
    return (
        <Link to="/raves" className="see-all-button">
            <h3>See All Raves</h3>
        </Link>
    );
};
