// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{process::ExitCode, sync::Mutex};

use anyhow::Result;
use config::{configure, Config};
use history::{setup_clipboard_watcher, ClipboardHistory};
use log::debug;
use shortcut::register_global_shortcut;
use tauri::{generate_handler, Manager, WindowEvent};
use tray::setup_tray;

mod config;
mod history;
mod shortcut;
mod tray;

fn main() -> Result<ExitCode> {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            debug!("app.setup()");

            debug!("create tray");
            setup_tray(app.handle())?;

            debug!("create clipboard history...");
            app.manage(Mutex::new(ClipboardHistory::new()));

            debug!("load config...");
            app.manage(Mutex::new(Config::default()));

            debug!("configuring app...");
            configure(app.handle());

            debug!("setup clipboard watcher...");
            setup_clipboard_watcher(app.handle().clone());

            debug!("register global shortcuts...");
            register_global_shortcut(app.handle());

            debug!("set activation policy...");
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            debug!(
                "running {}-{}...",
                app.package_info().name,
                app.package_info().version
            );
            Ok(())
        })
        .invoke_handler(generate_handler![
            history::history,
            history::paste,
            config::set_config,
            config::get_config
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::WindowEvent {
                label,
                event: WindowEvent::Focused(false),
                ..
            } => {
                app.get_webview_window(&label).unwrap().hide().unwrap();
            }
            _ => {}
        });

    Ok(ExitCode::SUCCESS)
}
