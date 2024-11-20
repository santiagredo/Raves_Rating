import { invoke } from "@tauri-apps/api/core";
import { getRaveType } from "../types/rave";

export default async function deleteRave(id: number): Promise<number> {
    let rave = getRaveType();
    rave.id = id;

    try {
        const response = await invoke<number>("delete_raves", {
            raves_collection: [rave],
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
