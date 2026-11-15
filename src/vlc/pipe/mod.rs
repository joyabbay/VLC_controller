use anyhow::Context;
use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command as ProcessCommand};
use tokio::task::JoinHandle;

use crate::command::VlcCommand;
use crate::config::Config;
use crate::vlc::VlcInterface;

pub struct PipeClient {
    child: Child,
    writer: ChildStdin,
    logger: JoinHandle<()>,
}

impl PipeClient {
    fn shutdown(&mut self) {
        self.logger.abort();
        let _ = self.child.start_kill();
    }
}

impl VlcInterface for PipeClient {
    fn connect(_config: Config) -> anyhow::Result<Self> {
        let mut child = ProcessCommand::new("vlc")
            .arg("--extraintf")
            .arg("rc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let writer = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("missing vlc stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("missing vlc stdout"))?;

        let logger = tokio::spawn(output_reader(stdout));

        Ok(Self {
            child,
            writer,
            logger,
        })
    }

    async fn execute(&mut self, cmd: VlcCommand) -> anyhow::Result<()> {
        let mut line = cmd.to_string();
        line.push('\n');

        tracing::debug!(target: "vlc", command = %cmd, "sending command");

        self.writer
            .write_all(line.as_bytes())
            .await
            .with_context(|| format!("failed to send `{cmd}` to VLC"))?;

        self.writer
            .flush()
            .await
            .context("failed to flush VLC stdin")?;

        Ok(())
    }

    async fn close(&mut self) -> anyhow::Result<()> {
        // Ask VLC to quit before forcing it.
        let _ = self.writer.write_all(b"quit\n").await;
        let _ = self.writer.flush().await;

        // SIGKILL and wait for the process so it does not stay a zombie.
        self.child.kill().await?;

        // Stop the logger and make the remaining cleanup idempotent.
        self.shutdown();
        Ok(())
    }

    async fn run(&mut self, mut rx: tokio::sync::mpsc::Receiver<VlcCommand>) -> anyhow::Result<()> {
        while let Some(command) = rx.recv().await {
            if matches!(command, VlcCommand::Quit) {
                break;
            }

            if let Err(error) = self.execute(command).await {
                tracing::error!("Unable to execute command: {error}");
            }
        }

        self.close().await?;
        Ok(())
    }
}
async fn output_reader(stdout: ChildStdout) {
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                tracing::info!(
                    target: "vlc",
                    "{}",
                    line.trim_end()
                );
            }
            Err(error) => {
                tracing::error!(
                    target: "vlc",
                    "Failed to read VLC output: {error}"
                );
                break;
            }
        }
    }
}
impl Drop for PipeClient {
    //Best effort drop as drop cant await
    fn drop(&mut self) {
        self.shutdown();
    }
}
