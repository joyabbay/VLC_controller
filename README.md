# VLC Controller

This repo implements audio controls for VLC player.

For now it is intended to support following commands:

```rust
    Play,
    Pause,
    Next,
    Previous,

    SetVolume(u32),
    VolumeUp,
    VolumeDown,

    Seek(u32),

    FullScreen(bool),

    Faster,
    Slower,
    NormalSpeed,

    Quit,
```

## Repo structure

```text
.
├── Cargo.toml
├── README.md
└── src
    ├── lib.rs                  # Crate root; exposes the modules below
    ├── command.rs              # VlcCommand enum 
    ├── config.rs               # Config struct (host, port, timeout, ...)
    ├── bin
    │   └── vlc_pipes.rs        # Test binary(For now,work in progress): reads commands from stdin
    └── vlc
        ├── mod.rs              # VlcInterface trait + PipeClient re-export
        └── pipe
            └── mod.rs          # PipeClient impl
```

## Build

Prerequisites:

- Rust with edition 2024 support (Rust 1.85 or newer)
- VLC installed and available as `vlc` on `PATH`

Build the library:

```bash
cargo build
```

Build and run the test binary:

```bash
cargo run --bin vlc_pipes
```

Enable command/log output:

```bash
RUST_LOG=debug cargo run --bin vlc_pipes
```

Then type commands into stdin:

```text
play
pause
next
prev
volume 256
volup
voldown
seek 30
fullscreen on
fullscreen off
faster
slower
normal
quit
```

Type `quit` (or press Ctrl-D) to close VLC and exit.

## Flow

```text
Microphone
        │
        ▼
  parse_command()
        │
        ▼
    VlcCommand
        │
        ▼
 mpsc::Sender<VlcCommand>
        │
        ▼
   PipeClient::run()
        │
        ▼
  PipeClient::execute()
        │
        │  Display -> "play\n", "volume 256\n", ...
        ▼
   VLC stdin  ──────────────►  VLC RC interface
                                     │
                                     ▼
   VLC stdout ◄──────────────  RC output/logs
        │
        ▼
  output_reader task -> tracing
```
