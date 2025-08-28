#[cfg(not(debug_assertions))]
use tauri::Manager;
#[cfg(not(debug_assertions))]
use tauri_plugin_autostart::ManagerExt;
#[cfg(not(debug_assertions))]
use tauri_plugin_store::StoreExt;

mod command;
use command::{remove_wallpaper, set_wallpaper};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build());

    // 安装插件
    builder = builder.plugin(
        tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
    );

    let app = builder
        .setup(|app| {
            #[cfg(not(debug_assertions))]
            {
                // 初始化 store
                let store = app.store("config.data")?;
                // 自动启动
                let config_autostart = store
                    .get("autostart")
                    .unwrap_or(serde_json::Value::Bool(true));
                let autostart_manager = app.autolaunch();
                let is_enabled = config_autostart.as_bool();
                log::info!("自启动状态: {:?}", is_enabled);

                if let Some(enabled) = is_enabled {
                    if enabled {
                        let _ = autostart_manager.enable();
                        log::info!("自启动设置为开启");
                    } else {
                        let _ = autostart_manager.disable();
                        log::info!("自启动设置为禁用");
                    }
                } else {
                    let _ = autostart_manager.enable();
                    log::warn!("无法检查自启动状态时，默认启用自启动");
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_wallpaper, remove_wallpaper])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // 阻止因窗口全部关闭而退出应用
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, code, .. } => {
            if code.is_none() {
                api.prevent_exit();
            }
        }
        _ => {}
    });
}
