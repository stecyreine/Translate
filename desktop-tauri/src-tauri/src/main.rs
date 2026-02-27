#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use std::process::{Command, Child};
use std::sync::Arc;
use std::sync::Mutex;

struct AppState {
  node_process: Arc<Mutex<Option<Child>>>,
}

#[tauri::command]
fn start_node_server(state: tauri::State<AppState>) -> Result<String, String> {
  let mut process_guard = state.node_process.lock().unwrap();
  
  // Si un processus existe déjà, ne pas relancer
  if process_guard.is_some() {
    return Ok("Serveur déjà en cours d'exécution".to_string());
  }

  let server_path = std::env::current_exe()
    .ok()
    .and_then(|path| path.parent().map(|p| p.to_path_buf()))
    .map(|mut path| {
      path.push("resources");
      path.push("server");
      path
    })
    .ok_or("Impossible de localiser le serveur".to_string())?;

  #[cfg(target_os = "windows")]
  let cmd = format!("{}\\server.exe", server_path.display());
  
  #[cfg(not(target_os = "windows"))]
  let cmd = format!("{}/server", server_path.display());

  let child = Command::new(&cmd)
    .spawn()
    .map_err(|e| format!("Erreur de lancement du serveur: {}", e))?;

  *process_guard = Some(child);
  Ok("Serveur Node.js démarré avec succès".to_string())
}

#[tauri::command]
fn stop_node_server(state: tauri::State<AppState>) -> Result<String, String> {
  let mut process_guard = state.node_process.lock().unwrap();
  
  if let Some(mut child) = process_guard.take() {
    child.kill()
      .map_err(|e| format!("Erreur arrêt serveur: {}", e))?;
  }
  
  Ok("Serveur arrêté".to_string())
}

#[tauri::command]
fn get_server_url() -> String {
  "https://localhost:3000".to_string()
}

fn main() {
  let app_state = AppState {
    node_process: Arc::new(Mutex::new(None)),
  };

  tauri::Builder::default()
    .manage(app_state)
    .invoke_handler(tauri::generate_handler![
      start_node_server,
      stop_node_server,
      get_server_url
    ])
    .setup(|app| {
      let state = app.state::<AppState>();
      
      // Lancer automatiquement le serveur au démarrage
      let node_process = state.node_process.clone();
      
      std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        if let Ok(mut guard) = node_process.lock() {
          if guard.is_none() {
            let server_binary = if cfg!(target_os = "windows") {
              "server.exe"
            } else if cfg!(target_os = "macos") {
              "server"
            } else {
              "server"
            };

            if let Ok(child) = Command::new(server_binary).spawn() {
              *guard = Some(child);
            }
          }
        }
      });

      Ok(())
    })
    .on_window_event(|event| {
      if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        api.prevent_close();
      }
    })
    .run(tauri::generate_context!())
    .expect("Erreur lors de l'exécution de l'application Tauri");
}
