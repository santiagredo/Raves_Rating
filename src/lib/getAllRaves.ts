import { getConditionsType } from "../types/conditions";
import { invoke } from "@tauri-apps/api/core";
import { RaveOverview } from "../types/raveOverview";

export async function getAllRaves(): Promise<RaveOverview[]> {
    let conditions = getConditionsType();

    try {
        const response = await invoke<RaveOverview[]>("select_raves_overview", {
            conditions,
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
