//! Playerctl Wrapper
//!
//! A `playerctl` rust wrapper allowing the control of players and current
//! playing track.
//!
//! See the [playerctl](https://github.com/altdesktop/playerctl) project for
//! more information.

use std::process::Command;
use thiserror::Error;

/// Playerctl errors.
#[derive(Debug, Error)]
pub enum PlayerctlError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Command error: {0}")]
    CommandError(String),
    #[error("Other error: {0}")]
    Other(String),
}
pub type Result<T> = std::result::Result<T, PlayerctlError>;

/// The current track status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackStatus {
    /// Track is playing.
    Playing,
    /// Track is paused.
    Paused,
    /// Track is stopped.
    Stopped,
}

/// Track metadata struct.
#[derive(Default)]
pub struct TrackMetadata {
    /// Track's title.
    pub title: String,
    /// Track's album.
    pub album: String,
    /// Track's artist.
    pub artist: String,
    /// Track's source URL.
    pub url: String,
    /// Track's length.
    pub length: String,
}

/// # Playerctl
///
/// Playerctl wrapper struct allowing to send commands and control
/// the player.
///
/// # Examples
///
/// ## Stop the current player
///
/// ```
/// Playerctl::stop().unwrap();
/// ```
///
/// ## Advance and rewind 10 seconds
///
/// ```
/// Playerctl::position(10.).unwrap();
/// Playerctl::position(-10.).unwrap();
/// ```
///
/// ## Get metadata
///
/// ```
/// let metadata = Playerctl::metadata().unwrap();
///
/// println!("Track title: {}", metadata.title);
/// ```
pub struct Playerctl;

impl Playerctl {
    /// Command the player to play.
    ///
    /// ```
    /// Playerctl::play().unwrap();
    /// ```
    pub fn play() -> Result<()> {
        run_command("play")?;
        Ok(())
    }

    /// Command the player to pause.
    ///
    /// ```
    /// Playerctl::pause().unwrap();
    /// ```
    pub fn pause() -> Result<()> {
        run_command("pause")?;
        Ok(())
    }

    /// Command the player to toggle between play/pause.
    ///
    /// ```
    /// Playerctl::play_pause().unwrap();
    /// ```
    pub fn play_pause() -> Result<()> {
        run_command("play-pause")?;
        Ok(())
    }

    /// Command the player to stop.
    ///
    /// ```
    /// Playerctl::stop().unwrap();
    /// ```
    pub fn stop() -> Result<()> {
        run_command("stop")?;
        Ok(())
    }

    /// Command the player to skip to the next track.
    ///
    /// ```
    /// Playerctl::next().unwrap();
    /// ```
    pub fn next() -> Result<()> {
        run_command("next")?;
        Ok(())
    }

    /// Command the player to skip to the previous track.
    ///
    /// ```
    /// Playerctl::previous().unwrap();
    /// ```
    pub fn previous() -> Result<()> {
        run_command("previous")?;
        Ok(())
    }

    /// Command the player to seek forward/backward OFFSET in seconds.
    ///
    /// ```
    /// Playerctl::position(10.).unwrap();
    /// ```
    pub fn position(secs: f32) -> Result<()> {
        if secs < 0. {
            run_command(&format!("position {}-", -secs))?;
        } else {
            run_command(&format!("position {}+", secs))?;
        }

        Ok(())
    }

    /// Set the volume to LEVEL from 0.0 to 1.0.
    ///
    /// ```
    /// Playerctl::volume(10.).unwrap();
    /// ```
    pub fn volume(percent: f32) -> Result<()> {
        if percent < 0. {
            run_command(&format!("volume {}-", -percent))?;
        } else {
            run_command(&format!("volume {}+", percent))?;
        }

        Ok(())
    }

    /// Get the play status of the player.
    ///
    /// ```
    /// Playerctl::status(10.).unwrap();
    /// ```
    pub fn status() -> Result<TrackStatus> {
        let status = run_command("status")?;

        match status.unwrap().as_str().trim() {
            "Playing" => Ok(TrackStatus::Playing),
            "Paused" => Ok(TrackStatus::Paused),
            _ => Ok(TrackStatus::Stopped),
        }
    }

    /// Get metadata information for the current track.
    ///
    /// ```
    /// let metadata = Playerctl::metadata().unwrap();
    ///
    /// println!("Title: {}", metadata.title);
    /// ```
    pub fn metadata() -> Result<TrackMetadata> {
        let title = run_command("metadata title")?;
        let album = run_command("metadata album")?;
        let artist = run_command("metadata artist")?;
        let url = run_command("metadata xesam:url")?;
        let length = run_command("metadata mpris:length")?;

        Ok(TrackMetadata {
            title: title.unwrap_or_default(),
            album: album.unwrap_or_default(),
            artist: artist.unwrap_or_default(),
            url: url.unwrap_or_default(),
            length: length.unwrap_or_default(),
        })
    }
}

/// Run a playerctl command.
///
/// ```
/// Playerctl::play().expect("Failed to run command");
/// ```
fn run_command(command: &str) -> Result<Option<String>> {
    let args: Vec<&str> = command.split_whitespace().collect();

    let output = Command::new("playerctl").args(args).output()?;

    if output.status.success() {
        Ok(Some(
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        ))
    } else {
        Err(PlayerctlError::CommandError(format!(
            "Command failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}
