import "./Home.css";
import { useEffect, useState } from "react";
import { getUpcomingRaves } from "../lib/getUpcomingRaves";
import { RaveCard } from "../components/RaveCard";
import { RaveOverview } from "../types/raveOverview";
import { getPreviousRaves } from "../lib/getPreviousRaves";
import { SeeAllLink } from "../components/SeeAllLink";

export default function Home() {
    const [upcomingRaves, setUpcomingRaves] = useState<RaveOverview[]>([]);
    const [previousRaves, setPreviousRaves] = useState<RaveOverview[]>([]);

    useEffect(() => {
        const fetchRaves = async () => {
            try {
                const fetchedUpcomingRaves = await getUpcomingRaves();
                setUpcomingRaves(fetchedUpcomingRaves);

                const fetchedPreviousRaves = await getPreviousRaves();
                setPreviousRaves(fetchedPreviousRaves);
            } catch (error) {
                console.log(error);
            }
        };

        fetchRaves();
    }, []);

    return (
        <main>
            <article>
                <h2 className="titles-link">Upcoming Raves</h2>

                <section>
                    {upcomingRaves.map((rave) => (
                        <RaveCard key={rave.id} rave={rave}></RaveCard>
                    ))}
                </section>
            </article>

            <article>
                <h2 className="titles-link">Previous Raves</h2>

                <section>
                    {previousRaves.map((rave) => (
                        <RaveCard key={rave.id} rave={rave}></RaveCard>
                    ))}
                </section>
            </article>

            <article className="article-see-all">
                <SeeAllLink></SeeAllLink>
            </article>
        </main>
    );
}
