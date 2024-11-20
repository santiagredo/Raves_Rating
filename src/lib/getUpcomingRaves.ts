import { getConditionsType } from "../types/conditions";
import { invoke } from "@tauri-apps/api/core";
import { RaveOverview } from "../types/raveOverview";

export async function getUpcomingRaves(): Promise<RaveOverview[]> {
    let conditions = getConditionsType();
    conditions.limit = 5;
    conditions.get_upcoming = true;

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
