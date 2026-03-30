use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

pub struct PtyInstance {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
}

pub struct PtyManager {
    instances: Arc<Mutex<HashMap<String, PtyInstance>>>,
}

impl PtyManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new PTY instance, spawn the default shell, and start
    /// a background reader thread that emits output events.
    pub fn spawn(&self, app_handle: AppHandle, cols: u16, rows: u16) -> Result<String, String> {
        let pty_system = native_pty_system();
        let id = Uuid::new_v4().to_string();

        let pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;

        // Determine the default shell
        let shell = if cfg!(target_os = "windows") {
            "powershell.exe".to_string()
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string())
        };

        let mut cmd = CommandBuilder::new(&shell);
        if !cfg!(target_os = "windows") {
            cmd.env("TERM", "xterm-256color");
        }

        // Spawn the shell process in the slave side of the pty
        pair.slave
            .spawn_command(cmd)
            .map_err(|e| e.to_string())?;

        // Obtain writer and reader from the master side
        let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
        let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;

        let event_id = id.clone();

        // Background thread: read PTY output and emit events to the frontend
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => {
                        let _ = app_handle.emit(&format!("pty-exit-{}", event_id), ());
                        break;
                    }
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buf[..n]).to_string();
                        let _ = app_handle.emit(&format!("pty-output-{}", event_id), data);
                    }
                    Err(_) => {
                        let _ = app_handle.emit(&format!("pty-exit-{}", event_id), ());
                        break;
                    }
                }
            }
        });

        let instance = PtyInstance {
            master: pair.master,
            writer,
        };
        self.instances
            .lock()
            .unwrap()
            .insert(id.clone(), instance);

        Ok(id)
    }

    /// Write data to an existing PTY.
    pub fn write(&self, id: &str, data: &str) -> Result<(), String> {
        let mut instances = self.instances.lock().unwrap();
        let instance = instances.get_mut(id).ok_or("PTY not found")?;
        instance
            .writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        instance.writer.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Resize an existing PTY.
    pub fn resize(&self, id: &str, cols: u16, rows: u16) -> Result<(), String> {
        let instances = self.instances.lock().unwrap();
        let instance = instances.get(id).ok_or("PTY not found")?;
        instance
            .master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Destroy a PTY instance (drops master + writer, causing the shell to exit).
    pub fn kill(&self, id: &str) -> Result<(), String> {
        let mut instances = self.instances.lock().unwrap();
        instances.remove(id);
        Ok(())
    }
}
