import { invoke } from "@tauri-apps/api/core";
import { Rave } from "../types/rave";

export default async function patchRaveDetail(
    id: number,
    name: string,
    location: number,
    rating: number,
    date: string
): Promise<number> {
    let rave: Rave = {
        id,
        name,
        location,
        rating,
        date,
    };

    try {
        const response = await invoke<number>("update_raves", {
            raves_collection: [rave],
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
