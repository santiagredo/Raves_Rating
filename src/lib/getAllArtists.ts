import { invoke } from "@tauri-apps/api/core";
import { Artist, getArtistType } from "../types/artist";
import { getConditionsType } from "../types/conditions";

export default async function getAllArtists(): Promise<Artist[]> {
    let artist = getArtistType();

    let conditions = getConditionsType();
    conditions.get_all = true;

    try {
        const response = await invoke<Artist[]>("select_artists", {
            artist,
            conditions,
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
