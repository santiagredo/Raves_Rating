import { invoke } from "@tauri-apps/api/core";
import { getConditionsType } from "../types/conditions";
import getRatingType, { Rating } from "../types/rating";

export default async function getAllRatings(): Promise<Rating[]> {
    let rating = getRatingType();
    let conditions = getConditionsType();
    conditions.get_all = true;

    try {
        const response = await invoke<Rating[]>("select_ratings", {
            rating,
            conditions,
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
