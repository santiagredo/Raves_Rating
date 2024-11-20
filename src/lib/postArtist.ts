import { invoke } from "@tauri-apps/api/core";
import { Artist, getArtistType } from "../types/artist";

export default async function postArtist(
    name: string,
    country: string
): Promise<Artist> {
    let artist = getArtistType();
    artist.name = name;
    artist.country = country;

    try {
        const response = await invoke<Artist>("insert_artist", {
            artist,
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
