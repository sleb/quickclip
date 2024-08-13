use std::{collections::VecDeque, fmt::Debug, sync::Mutex, thread};

use clipboard_rs::{
    Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
    ContentFormat,
};
use log::debug;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Debug)]
pub struct ClipboardHistory {
    inner: VecDeque<String>,
    size: usize,
}

impl Default for ClipboardHistory {
    fn default() -> Self {
        Self {
            inner: VecDeque::new(),
            size: 30,
        }
    }
}

impl ClipboardHistory {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    pub fn push(&mut self, item: String) {
        self.inner.push_front(item);
        if self.inner.len() > self.size {
            self.inner.pop_back();
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.size = new_size;
        self.inner.truncate(new_size);
    }
}

struct ClipboardChangeHandler {
    app: AppHandle,
    ctx: ClipboardContext,
}

impl ClipboardChangeHandler {
    fn new(app: AppHandle) -> Self {
        Self {
            app,
            ctx: ClipboardContext::new().expect("couldn't get clipboard context"),
        }
    }
}

impl ClipboardHandler for ClipboardChangeHandler {
    fn on_clipboard_change(&mut self) {
        debug!("clipboard change!");
        debug!("available formats: {:?}", self.ctx.available_formats());
        let history = self.app.state::<Mutex<ClipboardHistory>>();
        let mut history = history.lock().unwrap();
        debug!("history before: {history:?}");
        if self.ctx.has(ContentFormat::Text) {
            let text = self.ctx.get_text().unwrap();
            debug!("found text in clipboard: `{text}`");
            history.push(text);
            self.app.emit("history-updated", {}).unwrap();
        }
        debug!("history after: {history:?}");
    }
}

pub fn setup_clipboard_watcher(app: AppHandle) {
    let mut ctx =
        ClipboardWatcherContext::new().expect("couldn't create clipboard watcher context");
    ctx.add_handler(ClipboardChangeHandler::new(app));

    thread::spawn(move || {
        ctx.start_watch();
    });
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryItem {
    id: usize,
    value: String,
}

#[tauri::command]
pub async fn history(
    state: State<'_, Mutex<ClipboardHistory>>,
) -> Result<Vec<HistoryItem>, String> {
    let history = state.lock().map_err(|e| e.to_string())?;
    Ok(history
        .inner
        .iter()
        .enumerate()
        .map(|(id, value)| HistoryItem {
            id,
            value: value.clone(),
        })
        .collect())
}

#[tauri::command]
pub async fn paste(
    app: AppHandle,
    state: State<'_, Mutex<ClipboardHistory>>,
    id: usize,
) -> Result<(), String> {
    debug!("paste!");
    let mut history = state.lock().map_err(|e| e.to_string())?;
    debug!("history: {history:?}, id: {id}");

    let item = history
        .inner
        .remove(id)
        .ok_or(format!("couldn't find item `{id}` in history"))?;

    debug!("item: {item:?}");

    let ctx = ClipboardContext::new().unwrap();
    ctx.set_text(String::from(item)).unwrap();
    debug!("history after: {history:?}");
    app.emit("history-updated", {}).unwrap();

    Ok(())
}
