import { invoke } from "@tauri-apps/api/core";
import { getRaveArtistRelationType } from "../types/raveArtistRelation";

export default async function postRaveArtistRelation(
    rave_id: number,
    artist_id: number
): Promise<number> {
    let raveArtistRelation = getRaveArtistRelationType();
    raveArtistRelation.rave_id = rave_id;
    raveArtistRelation.artist_id = artist_id;

    try {
        console.log(raveArtistRelation);
        const response = await invoke<number>("insert_rave_artists", {
            relations_collection: [raveArtistRelation],
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
