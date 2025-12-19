use std::sync::Mutex;
pub use whis_core::RecordingState;
use whis_core::{AudioRecorder, Settings};

pub struct AppState {
    pub recording_state: Mutex<RecordingState>,
    pub recorder: Mutex<Option<AudioRecorder>>,
    pub settings: Mutex<Settings>,
}
