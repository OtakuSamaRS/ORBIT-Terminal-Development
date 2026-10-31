
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::{
    io::{Read, Write},
    sync::Mutex,
    thread,
};
use tauri::{AppHandle, Emitter, Manager, State};

struct TerminalSession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: std::sync::Arc<
        Mutex<Box<dyn portable_pty::Child + Send + Sync>>,
    >,
}
#[derive(Default)]
struct TerminalState {
    session: Mutex<Option<TerminalSession>>,
}

#[derive(serde::Serialize)]
struct DetectedShell {
    name: String,
    executable: String,
}

#[tauri::command]
fn detect_shells() -> Vec<DetectedShell> {
    #[cfg(windows)]
    let candidates = vec![
        ("PowerShell", "powershell.exe".to_string()),
        ("PowerShell 7", "pwsh.exe".to_string()),
        ("Command Prompt", "cmd.exe".to_string()),
        (
            "Git Bash",
            r"C:\Program Files\Git\bin\bash.exe".to_string(),
        ),
    ];

    #[cfg(not(windows))]
    let candidates = vec![
        ("Bash", "/bin/bash".to_string()),
        ("Zsh", "/bin/zsh".to_string()),
        ("POSIX Shell", "/bin/sh".to_string()),
        ("PowerShell", "pwsh".to_string()),
    ];

    let mut detected = Vec::new();

    for (name, executable) in candidates {
        let resolved = if std::path::Path::new(&executable).is_file() {
            Some(executable.clone())
        } else {
            #[cfg(windows)]
            let result = std::process::Command::new("where.exe")
                .arg(&executable)
                .output();

            #[cfg(not(windows))]
            let result = std::process::Command::new("which")
                .arg(&executable)
                .output();

            result.ok().and_then(|output| {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .map(|line| line.trim().to_string())
                        .filter(|line| !line.is_empty())
                } else {
                    None
                }
            })
        };

        if let Some(path) = resolved {
            detected.push(DetectedShell {
                name: name.to_string(),
                executable: path,
            });
        }
    }

    detected
}

#[tauri::command]


fn start_terminal(
    app: AppHandle,
    state: State<'_, TerminalState>,
    shell_executable: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let pty_system = native_pty_system();

    let pair = pty_system
        .openpty(PtySize {
            rows: rows.max(1),
            cols: cols.max(1),
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Could not create terminal: {e}"))?;

    // Only launch shells returned by our shell detector.
    let available_shells = detect_shells();

    let shell = available_shells
        .iter()
        .find(|shell| shell.executable == shell_executable)
        .ok_or_else(|| {
            format!("Shell is not available or is not supported: {shell_executable}")
        })?;

    let executable = shell.executable.clone();
    let shell_name = shell.name.as_str();

    let mut command = CommandBuilder::new(executable);

    #[cfg(windows)]
    {
        match shell_name {
            "PowerShell" => command.args(["-NoLogo", "-NoProfile"]),
            "PowerShell 7" => command.args(["-NoLogo", "-NoProfile"]),
            "Command Prompt" => command.arg("/Q"),
            "Git Bash" => command.args(["--login", "-i"]),
            _ => return Err(format!("Unsupported shell: {shell_name}")),
        }
    }

    #[cfg(not(windows))]
    {
        command.arg("-i");
    }

    let child = pair
        .slave
        .spawn_command(command)
        .map_err(|e| format!("Could not start shell: {e}"))?;

    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Could not read terminal output: {e}"))?;

    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("Could not write to terminal: {e}"))?;

    let child = std::sync::Arc::new(Mutex::new(child));

    let session = TerminalSession {
        master: pair.master,
        writer,
        child: child.clone(),
    };

    {
        let mut current = state
            .session
            .lock()
            .map_err(|e| format!("Terminal lock failed: {e}"))?;

        *current = Some(session);
    }

    // Monitor PowerShell independently of terminal output.
    let monitor_child = child.clone();
    let monitor_app = app.clone();

    thread::spawn(move || {
        loop {
            let exited = match monitor_child.lock() {
                Ok(mut child) => match child.try_wait() {
                    Ok(Some(_status)) => true,
                    Ok(None) => false,
                    Err(error) => {
                        eprintln!("Could not check PowerShell status: {error}");
                        false
                    }
                },
                Err(_) => break,
            };

            if exited {
                let _ = monitor_app.emit("terminal-exited", ());
                break;
            }

            thread::sleep(std::time::Duration::from_millis(250));
        }
    });

    // Read and forward terminal output.
    let output_app = app.clone();

    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(count) => {
                    let output =
                        String::from_utf8_lossy(&buffer[..count]).into_owned();

                    if output_app.emit("terminal-output", output).is_err() {
                        break;
                    }
                }
            }
        }
    });

    Ok(())
}


#[tauri::command]
fn write_terminal(
    state: State<'_, TerminalState>,
    data: String,
) -> Result<(), String> {
    let mut current = state
        .session
        .lock()
        .map_err(|e| format!("Terminal lock failed: {e}"))?;

    let session = current
        .as_mut()
        .ok_or_else(|| "Terminal has not been started".to_string())?;

    session
    .writer
    .write_all(data.as_bytes())
    .map_err(|e| format!("Could not write to shell: {e}"))?;

session
    .writer
    .flush()
    .map_err(|e| format!("Could not flush shell input: {e}"))?;

    Ok(())
}

#[tauri::command]
fn resize_terminal(
    state: State<'_, TerminalState>,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let current = state
        .session
        .lock()
        .map_err(|e| format!("Terminal lock failed: {e}"))?;

    let session = current
        .as_ref()
        .ok_or_else(|| "Terminal has not been started".to_string())?;

    session
        .master
        .resize(PtySize {
            rows: rows.max(1),
            cols: cols.max(1),
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Could not resize terminal: {e}"))?;

    Ok(())
}


#[tauri::command]
fn stop_terminal(state: State<'_, TerminalState>) -> Result<(), String> {
    let session = {
        let mut current = state
            .session
            .lock()
            .map_err(|e| format!("Terminal lock failed: {e}"))?;

        current.take()
    };

    if let Some(session) = session {
        let mut child = session
            .child
            .lock()
            .map_err(|e| format!("Shell process lock failed: {e}"))?;

        child
            .kill()
            .map_err(|e| format!("Could not stop shell: {e}"))?;

        child
            .wait()
            .map_err(|e| format!("Could not finish stopping shell: {e}"))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(TerminalState::default())
        .invoke_handler(tauri::generate_handler![
            detect_shells,
            start_terminal,
            write_terminal,
            resize_terminal,
            stop_terminal
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
    if let tauri::RunEvent::Exit = event {
        let state = app_handle.state::<TerminalState>();

        if let Ok(mut current) = state.session.lock() {
            if let Some(session) = current.take() {
                if let Ok(mut child) = session.child.lock() {
                     let _ = child.kill();
                     let _ = child.wait();
                     }
                } 
            }
        }
    });
}