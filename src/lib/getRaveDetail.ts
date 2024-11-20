import { invoke } from "@tauri-apps/api/core";
import { getRaveType } from "../types/rave";
import { RaveDetail } from "../types/raveDetail";

export async function getRaveDetail(id: string): Promise<RaveDetail> {
    let rave = getRaveType();
    rave.id = Number(id);

    try {
        const response = await invoke<RaveDetail>("select_rave_detail", {
            rave,
        });

        // console.log(response);
        return response;
    } catch (error) {
        throw error;
    }
}
