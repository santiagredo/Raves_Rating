use config::load_settings;
use types::{
    delete_artists, delete_expenses, delete_locations, delete_rave_artists, delete_raves,
    insert_artist, insert_artists, insert_expenses, insert_locations, insert_rave_artists,
    insert_raves, select_artists, select_expenses, select_locations, select_ratings,
    select_rave_detail, select_raves, select_raves_overview, update_artists, update_expenses,
    update_locations, update_raves,
};

mod types;

mod config;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    load_settings().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            insert_artist,
            insert_artists,
            select_artists,
            update_artists,
            delete_artists,
            insert_locations,
            select_locations,
            update_locations,
            delete_locations,
            select_ratings,
            insert_raves,
            insert_rave_artists,
            delete_rave_artists,
            select_raves,
            select_raves_overview,
            select_rave_detail,
            update_raves,
            delete_raves,
            insert_expenses,
            select_expenses,
            update_expenses,
            delete_expenses
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
