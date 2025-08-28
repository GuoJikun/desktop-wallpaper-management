use tauri::{AppHandle, Manager, WebviewWindow};

use wallpaper;
#[tauri::command]
pub fn set_wallpaper(app: AppHandle, window_label: String) {
    let w = app.get_webview_window(&window_label).unwrap();
    attach(&w);
}
#[tauri::command]
pub fn remove_wallpaper(app: AppHandle, window_label: String) {
    let w = app.get_webview_window(&window_label).unwrap();
    detach(&w);
}

fn attach(window: &WebviewWindow) {
    let hwnd = window.hwnd().unwrap();
    wallpaper::attach(hwnd);
    return;
}

fn detach(window: &WebviewWindow) {
    let hwnd = window.hwnd().unwrap();
    wallpaper::detach(hwnd);
    return;
}
