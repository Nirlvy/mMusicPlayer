use crate::ws;

use lazy_static::lazy_static;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    {Arc, Mutex},
};

lazy_static! {
    static ref CONN: Mutex<Option<mpd::Client>> = Mutex::new(None);
    static ref ADDRESS: Arc<Mutex<String>> = Arc::new(Mutex::new("localhost".to_owned()));
}

static PORT: AtomicU16 = AtomicU16::new(6600);

fn get_connection() -> Result<mpd::Client, String> {
    let conn = CONN.lock().unwrap().take();

    match conn {
        Some(conn) => Ok(conn),
        None => {
            connect(&*ADDRESS.lock().unwrap(), PORT.load(Ordering::Relaxed))?;
            get_connection()
        }
    }
}

pub fn get_status() -> Result<String, String> {
    let mut conn = get_connection().map_err(|e| e.to_string())?;
    let status = conn.status().map_err(|e| e.to_string())?;
    let message = serde_json::to_string(&status).map_err(|e| e.to_string())?;

    Ok(message)
}

#[tauri::command]
pub fn connect(address: &str, port: u16) -> Result<(), String> {
    *ADDRESS.lock().unwrap() = address.to_string();
    PORT.store(port, Ordering::Relaxed);

    let conn = mpd::Client::connect(format!("{}:{}", address, port)).map_err(|e| e.to_string());
    match conn {
        Ok(conn) => {
            let mut new_conn = CONN.lock().unwrap();
            *new_conn = Some(conn);
        }
        Err(e) => return Err(format!("Failed to establish MPD connection: {}", e)),
    };

    tokio::spawn(async {
        println!("test");
        ws::init().await;
    });

    Ok(())
}
