use crate::ipc;
use anyhow::Result;

pub fn run(hotkey: Option<String>) -> Result<()> {
    // Determine hotkey to use
    let hotkey_str = if let Some(h) = hotkey {
        h
    } else {
        // Try to read from running service's PID file
        match ipc::read_hotkey_from_pid_file() {
            Ok(h) => {
                println!("Preserving hotkey from running service: {}", h);
                h
            }
            Err(_) => {
                // Fall back to default
                let default = "ctrl+alt+w".to_string();
                println!("Using default hotkey: {}", default);
                default
            }
        }
    };

    // Stop the service if running
    if ipc::is_service_running() {
        let mut client = ipc::IpcClient::connect()?;
        let _ = client.send_message(ipc::IpcMessage::Stop)?;
        println!("Service stopped");

        // Wait a moment for graceful shutdown
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    // Start with the determined hotkey
    crate::commands::start::run(hotkey_str)
}
