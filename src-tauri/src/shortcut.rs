use log::debug;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::Shortcut;
use tauri_plugin_global_shortcut::ShortcutState;

pub fn register_global_shortcut(app: &AppHandle) {
    let show_list_shortcut = "super+alt+v".parse::<Shortcut>().unwrap();

    app.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts([show_list_shortcut.clone()])
            .unwrap()
            .with_handler(move |app, shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if shortcut.matches(show_list_shortcut.mods, show_list_shortcut.key) {
                        debug!("global shortcut!");
                        let window = app.get_webview_window("main").unwrap();

                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                }
            })
            .build(),
    )
    .unwrap();
}
