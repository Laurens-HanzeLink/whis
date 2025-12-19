use anyhow::{Context, Result};
use fantoccini::{Client, ClientBuilder, Locator};
use serde_json::json;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

pub struct TauriDriver {
    process: Option<Child>,
    client: Option<Client>,
}

impl TauriDriver {
    pub async fn new(app_binary: &str) -> Result<Self> {
        // Start tauri-driver
        let process = Command::new("tauri-driver")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to start tauri-driver. Is it installed? Run: cargo install tauri-driver --locked")?;

        // Wait for driver to be ready
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Build capabilities with the app binary path
        let caps = json!({
            "tauri:options": {
                "application": app_binary
            }
        });

        // Connect to tauri-driver
        let client = ClientBuilder::native()
            .capabilities(caps.as_object().unwrap().clone())
            .connect("http://localhost:4444")
            .await
            .context("Failed to connect to tauri-driver")?;

        // Set window size for consistent screenshots (tall enough to show full settings)
        client
            .set_window_size(900, 800)
            .await
            .context("Failed to set window size")?;

        Ok(Self {
            process: Some(process),
            client: Some(client),
        })
    }

    fn client(&self) -> Result<&Client> {
        self.client
            .as_ref()
            .context("WebDriver client already closed")
    }

    pub async fn screenshot(&self, path: &str) -> Result<()> {
        let screenshot = self.client()?.screenshot().await?;
        std::fs::write(path, screenshot)?;
        Ok(())
    }

    pub async fn navigate(&self, route: &str) -> Result<()> {
        // Tauri uses hash-based routing
        let url = format!("tauri://localhost/#{}", route);
        self.client()?.goto(&url).await?;
        // Wait for navigation and render
        tokio::time::sleep(Duration::from_millis(500)).await;
        Ok(())
    }

    pub async fn click(&self, selector: &str) -> Result<()> {
        let element = self
            .client()?
            .find(Locator::Css(selector))
            .await
            .context(format!("Element not found: {}", selector))?;
        element.click().await?;
        // Wait for animation
        tokio::time::sleep(Duration::from_millis(300)).await;
        Ok(())
    }

    pub async fn scroll_to(&self, selector: &str) -> Result<()> {
        let script = format!(
            r#"document.querySelector('{}')?.scrollIntoView({{ behavior: 'instant', block: 'center' }})"#,
            selector
        );
        self.client()?
            .execute(&script, vec![])
            .await
            .context("Failed to scroll")?;
        tokio::time::sleep(Duration::from_millis(200)).await;
        Ok(())
    }

    pub async fn select_option(&self, dropdown_selector: &str, option_text: &str) -> Result<()> {
        // Click dropdown to open it
        self.click(dropdown_selector).await?;

        // Find and click option by text using JavaScript
        let script = format!(
            r#"
            const options = document.querySelectorAll('.select-option');
            for (const opt of options) {{
                if (opt.textContent.includes('{}')) {{
                    opt.click();
                    break;
                }}
            }}
            "#,
            option_text
        );
        self.client()?
            .execute(&script, vec![])
            .await
            .context("Failed to execute select script")?;

        // Wait for selection to apply
        tokio::time::sleep(Duration::from_millis(300)).await;
        Ok(())
    }

    pub async fn close(mut self) -> Result<()> {
        if let Some(client) = self.client.take() {
            client.close().await?;
        }
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
        }
        Ok(())
    }
}

impl Drop for TauriDriver {
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
        }
    }
}
