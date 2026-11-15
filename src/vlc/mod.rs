use crate::{command::VlcCommand, config::Config};
mod pipe;

pub use pipe::PipeClient;
use tokio::sync::mpsc;

#[allow(async_fn_in_trait)]
pub trait VlcInterface: Send + Sync {
    fn connect(config: Config) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn execute(&mut self, cmd: VlcCommand) -> anyhow::Result<()>;
    async fn close(&mut self) -> anyhow::Result<()>;
    async fn run(&mut self, rx: mpsc::Receiver<VlcCommand>) -> anyhow::Result<()>;
}
