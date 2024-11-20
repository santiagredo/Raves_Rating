import { useEffect, useState } from "react";
import { RaveOverview } from "../types/raveOverview";
import { getAllRaves } from "../lib/getAllRaves";
import { RaveCard } from "../components/RaveCard";
import { Link } from "react-router-dom";
import "./Raves.css";

export default function Raves() {
    const [allRaves, setAllRaves] = useState<RaveOverview[]>([]);

    useEffect(() => {
        const fetchRaves = async () => {
            try {
                const fetchedAllRaves = await getAllRaves();
                setAllRaves(fetchedAllRaves);
            } catch (error) {
                console.log(error);
            }
        };

        fetchRaves();
    }, []);

    return (
        <main>
            <article>
                <h2>All Raves</h2>

                <section className="section-add-rave">
                    <Link to={"/add-rave"} className="link-add-rave">
                        <h3>Add New Rave</h3>
                    </Link>
                </section>

                <section>
                    {allRaves.map((rave) => (
                        <RaveCard rave={rave}></RaveCard>
                    ))}
                </section>
            </article>
        </main>
    );
}
