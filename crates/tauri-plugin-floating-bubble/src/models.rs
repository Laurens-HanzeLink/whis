use serde::{Deserialize, Serialize};

/// Color configuration for bubble states.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BubbleColors {
    /// Background color (hex string, e.g., "#1C1C1C").
    /// Default: "#1C1C1C" (dark)
    #[serde(default = "default_background_color")]
    pub background: String,

    /// Icon color for idle state (hex string).
    /// Default: "#FFFFFF" (white)
    #[serde(default = "default_idle_color")]
    pub idle: String,

    /// Icon color for recording state (hex string).
    /// Default: "#FF4444" (red)
    #[serde(default = "default_recording_color")]
    pub recording: String,

    /// Icon color for processing state (hex string).
    /// Default: "#FFD633" (gold)
    #[serde(default = "default_processing_color")]
    pub processing: String,
}

fn default_background_color() -> String {
    "#1C1C1C".to_string()
}

fn default_idle_color() -> String {
    "#FFFFFF".to_string()
}

fn default_recording_color() -> String {
    "#FF4444".to_string()
}

fn default_processing_color() -> String {
    "#FFD633".to_string()
}

/// Options for configuring the floating bubble.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BubbleOptions {
    /// Size of the bubble in dp (density-independent pixels).
    /// Default: 60
    #[serde(default = "default_size")]
    pub size: i32,

    /// Initial X position of the bubble.
    /// Default: 0 (left edge)
    #[serde(default)]
    pub start_x: i32,

    /// Initial Y position of the bubble.
    /// Default: 100
    #[serde(default = "default_start_y")]
    pub start_y: i32,

    /// Android drawable resource name for the icon (without "R.drawable." prefix).
    /// If not specified, uses the plugin's default icon.
    /// Example: "ic_my_app_logo"
    #[serde(default)]
    pub icon_resource_name: Option<String>,

    /// Color configuration for different bubble states.
    #[serde(default)]
    pub colors: Option<BubbleColors>,
}

fn default_size() -> i32 {
    60
}

fn default_start_y() -> i32 {
    100
}

/// Response from visibility check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibilityResponse {
    pub visible: bool,
}

/// Response from permission check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub granted: bool,
}

/// Options for setting recording state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingOptions {
    pub recording: bool,
}

/// Options for setting bubble state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateOptions {
    pub state: String,
}
