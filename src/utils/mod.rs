pub mod config;
pub mod fzf;
pub mod rofi;
pub mod image_preview;
pub mod players;
pub mod ffmpeg;

#[derive(thiserror::Error, Debug)]
pub enum SpawnError {
    #[error("Failed to spawn process: {0}")]
    IOError(std::io::Error),
}
