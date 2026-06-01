use tauri_plugin_sql::{Migration, MigrationKind};
use tauri_specta::{collect_commands, Builder};

/// A serializable error type for fallible Tauri commands.
/// The frontend receives the `message` string (and a typed shape via tauri-specta).
#[derive(Debug, thiserror::Error, serde::Serialize, specta::Type)]
#[error("{message}")]
pub struct CommandError {
    pub message: String,
}

impl CommandError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Example of a fallible command with a typed error.
#[tauri::command]
#[specta::specta]
fn safe_divide(a: f64, b: f64) -> Result<f64, CommandError> {
    if b == 0.0 {
        return Err(CommandError::new("cannot divide by zero"));
    }
    Ok(a / b)
}

/// Build the tauri-specta command registry. Shared by `run()` and the
/// `export_bindings` test so generated TypeScript always matches the app.
fn specta_builder() -> Builder {
    Builder::<tauri::Wry>::new().commands(collect_commands![greet, safe_divide])
}

/// SQLite migrations applied to `sqlite:app.db` on startup.
fn migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
              );",
        kind: MigrationKind::Up,
    }]
}

/// Writes the typed TypeScript client to `src/lib/bindings.ts`.
/// Only compiled in debug builds; the `@ts-nocheck` header keeps the
/// generated file out of the project's lint/type-check.
#[cfg(debug_assertions)]
fn export_bindings(builder: &Builder) {
    builder
        .export(
            specta_typescript::Typescript::default()
                .header("// @ts-nocheck\n/* eslint-disable */\n"),
            "../src/lib/bindings.ts",
        )
        .expect("failed to export typescript bindings");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    // Regenerate TypeScript bindings on every dev build.
    #[cfg(debug_assertions)]
    export_bindings(&builder);

    let mut app = tauri::Builder::default();

    // Desktop-only plugins. `single_instance` must be registered first.
    #[cfg(desktop)]
    {
        use tauri::Manager;
        app = app
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_focus();
                }
            }))
            .plugin(tauri_plugin_window_state::Builder::default().build())
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_process::init());
    }

    app.plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:app.db", migrations())
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generates `src/lib/bindings.ts`. Run with `cargo test export_bindings`.
    #[test]
    fn export_bindings() {
        super::export_bindings(&specta_builder());
    }
}
