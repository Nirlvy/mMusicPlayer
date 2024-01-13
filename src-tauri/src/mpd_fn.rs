use crate::ws;

use lazy_static::lazy_static;
use mpd::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref ADDRESS: Arc<Mutex<String>> = Arc::new(Mutex::new("localhost".to_string()));
    static ref PORT: Arc<Mutex<u16>> = Arc::new(Mutex::new(6600));
}

fn get_connection() -> Result<Client, String> {
    let address = Arc::clone(&ADDRESS);
    let address = address.lock().unwrap();
    let address = address.as_str();

    let port = Arc::clone(&PORT);
    let port = *port.lock().unwrap();

    let conn = Client::connect(format!("{}:{}", address, port)).map_err(|e| e.to_string());
    match conn {
        Ok(conn) => Ok(conn),
        Err(e) => Err(format!("mpd connect error: {}", e)),
    }
}

pub fn get_status() -> Result<String, String> {
    let conn = get_connection();
    match conn {
        Ok(conn) => {
            let mut conn = conn;
            let status = conn.status().map_err(|e| e.to_string())?;
            let message = serde_json::to_string(&status).map_err(|e| e.to_string())?;
            log::debug!("{:?}", message);

            Ok(message)
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub fn connect(address: &str, port: u16) -> Result<String, String> {
    let conn = Client::connect(format!("{}:{}", address, port)).map_err(|e| e.to_string());
    match conn {
        Ok(_) => {
            let mut ads = ADDRESS.lock().unwrap();
            *ads = address.to_string();

            let mut pts = PORT.lock().unwrap();
            *pts = port;

            tokio::spawn(ws::init());

            Ok("mpd connect success".to_string())
        }
        Err(e) => Err(format!("mpd connect error: {}", e)),
    }
}

#[tauri::command]
pub fn get_currentsong() -> Result<String, String> {
    let conn = get_connection();
    match conn {
        Ok(conn) => {
            let mut conn = conn;
            let status = conn.currentsong().map_err(|e| e.to_string())?;
            let message = serde_json::to_string(&status).map_err(|e| e.to_string())?;

            Ok(message)
        }
        Err(e) => Err(e),
    }
}

#[derive(Serialize, Deserialize)]
pub enum States {
    Next,
    Prev,
    Stop,
    Toggle,
}

#[tauri::command]
pub fn set_playstate(state: States)->Result<(),String>{
    let conn = get_connection();
    match conn {
        Ok(conn) => {
            let mut conn = conn;
            match state {
                States::Next => {
                    conn.next().map_err(|e| e.to_string())?;
                }
                States::Prev => {
                    conn.prev().map_err(|e| e.to_string())?;
                }
                States::Stop => {
                    conn.stop().map_err(|e| e.to_string())?;
                }
                States::Toggle => {
                    conn.toggle_pause().map_err(|e| e.to_string())?;
                }
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}