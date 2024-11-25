// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    menu::{AboutMetadata, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager,
};
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            let settings = MenuItemBuilder::new("Settings...")
                .id("settings")
                .accelerator("CmdOrCtrl+,")
                .build(app)?;
            
            // my custom app submenu
            let app_submenu = SubmenuBuilder::new(app, "App")
                .about(Some(AboutMetadata {
                    ..Default::default()
                }))
                .separator()
                .item(&settings)
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .quit()
                .build()?;
            
                let menu = MenuBuilder::new(app)
                .items(&[
                    &app_submenu,
                ])
                .build()?;

                app.set_menu(menu);
                app.on_menu_event(move |app, event| {
                    if event.id() == settings.id(){
                        let _event = app.emit("custom-event", "/settings");
                    }
                });
                Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
