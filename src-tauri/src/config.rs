use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_autostart::ManagerExt;

use crate::history::ClipboardHistory;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    autostart: bool,
    history_limit: usize,
}

#[tauri::command]
pub async fn set_config(
    app: AppHandle,
    state: State<'_, Mutex<Config>>,
    config: Config,
) -> Result<(), String> {
    *state.lock().unwrap() = config;
    configure(&app);
    app.emit("config-updated", ()).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_config(state: State<'_, Mutex<Config>>) -> Result<Config, String> {
    let config = state.lock().unwrap();
    Ok(config.clone())
}

pub fn configure(app: &AppHandle) {
    let config_state = app.state::<Mutex<Config>>();
    let config = config_state.lock().unwrap();
    if config.autostart {
        app.autolaunch().enable().unwrap();
    } else {
        app.autolaunch().disable().unwrap();
    }

    let history_state = app.state::<Mutex<ClipboardHistory>>();
    let mut history = history_state.lock().unwrap();
    history.resize(config.history_limit);
}
