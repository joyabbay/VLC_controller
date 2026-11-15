use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;

use vlc_controller::command::VlcCommand;
use vlc_controller::config::Config;
use vlc_controller::vlc::{PipeClient, VlcInterface};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel::<VlcCommand>(32);

    let mut client = PipeClient::connect(Config::default())?;

    // Temporary: read commands from stdin instead of voice input.
    let input_task = read_commands(tx);
    let client_task = client.run(rx);

    tokio::try_join!(input_task, client_task)?;

    Ok(())
}

//TODO: Remove this in future this is for testing
async fn read_commands(tx: mpsc::Sender<VlcCommand>) -> Result<()> {
    println!(
        "Commands:\n  play\n  pause\n  next\n  prev\n  volume <0-1024>\n  volup\n  voldown\n  seek <seconds>\n  fullscreen <on|off>\n  faster\n  slower\n  normal\n  quit"
    );

    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let Some(command) = parse_command(line) else {
            eprintln!("Unknown command: {line}");
            continue;
        };

        let is_quit = matches!(command, VlcCommand::Quit);

        if tx.send(command).await.is_err() {
            break;
        }

        if is_quit {
            break;
        }
    }

    Ok(())
}

//TODO: Remove this in future this is for testing
fn parse_command(input: &str) -> Option<VlcCommand> {
    let mut parts = input.split_whitespace();
    let command = parts.next()?.to_ascii_lowercase();
    let arg = parts.next();

    Some(match command.as_str() {
        "play" => VlcCommand::Play,
        "pause" => VlcCommand::Pause,
        "next" => VlcCommand::Next,
        "prev" | "previous" => VlcCommand::Previous,

        "volume" => VlcCommand::SetVolume(arg?.parse().ok()?),
        "volup" => VlcCommand::VolumeUp,
        "voldown" => VlcCommand::VolumeDown,

        "seek" => VlcCommand::Seek(arg?.parse().ok()?),

        "fullscreen" => match arg? {
            "on" => VlcCommand::FullScreen(true),
            "off" => VlcCommand::FullScreen(false),
            _ => return None,
        },

        "faster" => VlcCommand::Faster,
        "slower" => VlcCommand::Slower,
        "normal" => VlcCommand::NormalSpeed,

        "quit" | "exit" => VlcCommand::Quit,

        _ => return None,
    })
}
