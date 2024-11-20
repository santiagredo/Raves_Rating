import { invoke } from "@tauri-apps/api/core";
import { getConditionsType } from "../types/conditions";
import getLocationType, { Location } from "../types/location";

export default async function getAllLocations(): Promise<Location[]> {
    let location = getLocationType();

    let conditions = getConditionsType();
    conditions.get_all = true;

    try {
        const response = await invoke<Location[]>("select_locations", {
            location,
            conditions,
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
