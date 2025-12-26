use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
pub use whis_core::RecordingState;

#[derive(Clone)]
pub struct AppState {
    pub recording_state: Arc<Mutex<RecordingState>>,
    pub audio_tx: Arc<Mutex<Option<mpsc::Sender<Vec<f32>>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            recording_state: Arc::new(Mutex::new(RecordingState::Idle)),
            audio_tx: Arc::new(Mutex::new(None)),
        }
    }
}
