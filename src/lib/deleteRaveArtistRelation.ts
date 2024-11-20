import { invoke } from "@tauri-apps/api/core";
import { getRaveArtistRelationType } from "../types/raveArtistRelation";

export default async function deleteRaveArtistRelation(
    rave_id: number,
    artist_id: number
): Promise<number> {
    let relation = getRaveArtistRelationType();
    relation.rave_id = rave_id;
    relation.artist_id = artist_id;

    try {
        const response = await invoke<number>("delete_rave_artists", {
            relations_collection: [relation],
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
