import { invoke } from "@tauri-apps/api/core";
import { Artist, getArtistType } from "../types/artist";
import { getConditionsType } from "../types/conditions";

export async function getArtistDetail(id: number): Promise<Artist> {
    let artist = getArtistType();
    artist.id = id;

    let conditions = getConditionsType();

    try {
        const response = await invoke<Artist[]>("select_artists", {
            artist,
            conditions,
        });

        // console.log(response);
        return response[0];
    } catch (error) {
        throw error;
    }
}
