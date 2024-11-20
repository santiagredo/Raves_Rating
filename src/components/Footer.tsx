import leftArrow from "../assets/left-arrow.svg";
import add from "../assets/add.svg";
import "./Footer.css";
import { useEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

export default function Footer() {
    const [hasHistory, setHasHistory] = useState(false);
    const [path, setPath] = useState("");

    const location = useLocation();
    const navigate = useNavigate();

    useEffect(() => {
        // Check if there's history when the path changes
        setHasHistory(window.history.length > 1);
        setPath(location.pathname);
    }, [location]);

    const goBack = () => {
        if (hasHistory) {
            window.history.back();
        }
    };

    const goAddPage = () => {
        navigate("/add");
    };

    return (
        <footer>
            <nav>
                <div>
                    <button
                        onClick={goBack}
                        style={{
                            opacity: path != "/" ? 1 : 0.5,
                        }}
                        className={path !== "/" ? "hover-blue hover-white" : ""}
                    >
                        <img src={leftArrow} alt="Back arrow" />
                    </button>
                </div>

                <div>
                    <button
                        onClick={goAddPage}
                        className={"hover-white hover-blue"}
                    >
                        <img src={add} alt="Add sign button" />
                    </button>
                </div>
            </nav>
        </footer>
    );
}
