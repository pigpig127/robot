use tauri::{AboutMetadata, CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowMenuEvent};

use crate::utils;

pub fn init_system_menu() -> Menu {
    let name = "Robot";

    let app_menu = Submenu::new(
        name,
        Menu::with_items([
            #[cfg(target_os = "macos")]
            MenuItem::About(name.into(), AboutMetadata::default()).into(),
            #[cfg(not(target_os = "macos"))]
            CustomMenuItem::new("about", "关于 Robot").into(),
            CustomMenuItem::new("check_update", "检查更新").into(),
        ]),
    );

    Menu::new().add_submenu(app_menu)
}

pub fn menu_event(event: WindowMenuEvent<tauri::Wry>) {
    let win = Some(event.window()).unwrap();
    let app = win.app_handle();
    let menu_id = event.menu_item_id();

    match menu_id {
        // App
        "about" => {}
        "check_update" => {
            utils::updater::run_check_update(app, true, Some(true));
        }
        _ => (),
    }
}
