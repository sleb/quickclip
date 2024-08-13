use anyhow::Result;
use tauri::{
    menu::{MenuBuilder, MenuEvent},
    AppHandle, Manager,
};

pub fn setup_tray(app_handle: &AppHandle) -> Result<()> {
    let tray = app_handle
        .tray_by_id("main")
        .expect("Couldn't find tray `main`");

    tray.set_menu(Some(
        MenuBuilder::new(app_handle)
            .text("about", "About")
            .text("prefs", "Preferences")
            .separator()
            .text("quit", "Quit")
            .build()
            .expect("couldn't build menu item"),
    ))
    .unwrap();

    tray.on_menu_event(|app_handle, event| match event {
        MenuEvent { id } => match id.as_ref() {
            "quit" => app_handle.exit(0),
            "about" => app_handle
                .get_webview_window("about")
                .unwrap()
                .show()
                .unwrap(),
            "prefs" => app_handle
                .get_webview_window("prefs")
                .unwrap()
                .show()
                .unwrap(),
            _ => {}
        },
    });

    Ok(())
}
