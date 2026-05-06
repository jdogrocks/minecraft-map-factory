use crate::config::InstallerConfig;
use std::path::Path;
use tracing::{info, warn};

/// Calls the local-server install script after a successful publish.
pub struct Installer<'a> {
    config: &'a InstallerConfig,
}

impl<'a> Installer<'a> {
    pub fn new(config: &'a InstallerConfig) -> Self {
        Self { config }
    }

    /// Run the install script with `published_dir` and the configured
    /// `server_dir`.  Logs a warning and returns `Ok(())` when the script
    /// is absent so a mis-configured path doesn't abort an otherwise
    /// successful pipeline run.  Returns an error if `server_dir` is not
    /// set — callers must configure it explicitly in pipeline.toml.
    pub async fn install(
        &self,
        published_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let script = &self.config.script;

        if !script.exists() {
            warn!(
                script = %script.display(),
                "Install script not found — skipping auto-install"
            );
            return Ok(());
        }

        let server_dir = self.config.server_dir.as_ref().ok_or(
            "auto_install is enabled but installer.server_dir is not set in pipeline.toml",
        )?;

        info!(
            published_dir = %published_dir.display(),
            server_dir = %server_dir.display(),
            script = %script.display(),
            "Auto-installing map to local Minecraft server"
        );

        let status = tokio::process::Command::new(script)
            .arg(published_dir)
            .arg(server_dir)
            .status()
            .await?;

        if status.success() {
            info!("Map installed to local server successfully");
            Ok(())
        } else {
            Err(format!("install script exited with {status}").into())
        }
    }
}
