mod capture;
mod driver;

use anyhow::{Context, Result};
use capture::CaptureConfig;
use clap::Parser;
use driver::TauriDriver;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "whis-screenshots")]
#[command(about = "Automated screenshot capture for whis-desktop")]
struct Args {
    /// Output directory for screenshots
    #[arg(short, long, default_value = "./screenshots")]
    output: String,

    /// Path to whis-desktop binary
    #[arg(short, long)]
    binary: Option<String>,

    /// Only capture specific view (home, settings, presets, shortcut, about)
    #[arg(short, long)]
    view: Option<String>,
}

fn find_binary() -> Result<PathBuf> {
    // Try to find the binary in common locations
    let candidates = [
        // Release build
        PathBuf::from("target/release/whis-desktop"),
        // Debug build
        PathBuf::from("target/debug/whis-desktop"),
        // From workspace root
        PathBuf::from("../target/release/whis-desktop"),
        PathBuf::from("../target/debug/whis-desktop"),
    ];

    for candidate in candidates {
        if candidate.exists() {
            return Ok(candidate.canonicalize()?);
        }
    }

    anyhow::bail!(
        "Could not find whis-desktop binary. Please specify with --binary or build with: cargo build -p whis-desktop"
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Find or validate binary path
    let binary_path = match args.binary {
        Some(path) => {
            let path = PathBuf::from(&path);
            if !path.exists() {
                anyhow::bail!("Binary not found: {}", path.display());
            }
            path.canonicalize()?
        }
        None => find_binary()?,
    };

    println!("Using binary: {}", binary_path.display());
    println!("Output directory: {}", args.output);

    // Start the driver
    println!("\nStarting tauri-driver...");
    let driver = TauriDriver::new(binary_path.to_str().unwrap())
        .await
        .context("Failed to initialize WebDriver")?;

    println!("Connected to whis-desktop\n");

    // Capture screenshots
    let config = CaptureConfig {
        output_dir: args.output.clone(),
        view_filter: args.view,
    };

    let captured = capture::capture_all(&driver, &config).await?;

    // Clean up
    driver.close().await?;

    // Report results
    println!("\nCaptured {} screenshots:", captured.len());
    for name in &captured {
        println!("  - {}/{}", args.output, name);
    }

    Ok(())
}
