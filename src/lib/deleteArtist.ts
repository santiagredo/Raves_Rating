import { invoke } from "@tauri-apps/api/core";
import { getArtistType } from "../types/artist";

export default async function deleteArtist(id: number): Promise<number> {
    let artist = getArtistType();
    artist.id = id;

    try {
        const response = await invoke<number>("delete_artists", {
            artists_collection: [artist],
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
