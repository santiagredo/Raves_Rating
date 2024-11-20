import { Link } from "react-router-dom";
import "./Add.css";

export default function Add() {
    return (
        <main>
            <article>
                <h2>Manage Items</h2>

                <section>
                    <Link to={"/add-rave"} className="link">
                        <h3>Add Rave</h3>
                    </Link>

                    <Link to={""} className="link">
                        <h3>Manage Locations</h3>
                    </Link>

                    <Link to={""} className="link">
                        <h3>Manage Artists</h3>
                    </Link>

                    <Link to={""} className="link">
                        <h3>Manage Ratings</h3>
                    </Link>
                </section>
            </article>
        </main>
    );
}
