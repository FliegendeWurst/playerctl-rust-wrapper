//! Playerctl Wrapper
//!
//! A `playerctl` rust wrapper allowing the control of players and current
//! playing track.
//!
//! See the [playerctl](https://github.com/altdesktop/playerctl) project for
//! more information.

use std::{collections::HashMap, num::ParseIntError, process::Command, string::FromUtf8Error};
use thiserror::Error;

/// Playerctl errors.
#[derive(Debug, Error)]
pub enum PlayerctlError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Command error: {0}")]
    CommandError(String),
    #[error("Failed to parse track length: {0}")]
    ParseLengthError(#[from] ParseIntError),
    #[error("Failed to parse metadata URL: {0}")]
    ParseUrlError(#[from] FromUtf8Error),
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

/// Player metadata struct.
/// 
/// There are many more xesam properties.
/// 
/// Reference: [MPRIS v2 metadata guidelines](https://freedesktop.org/wiki/Specifications/mpris-spec/metadata/)
#[derive(Default, Debug, PartialEq, Eq)]
pub struct PlayerMetadata {
    /// Example values: `'/org/mpris/MediaPlayer2/firefox'` (Firefox playing a Youtube video), `'/63'` (mpv, playlist item 63)
    pub mpris_trackid: Option<String>,
    /// Album/thumbnail art.
    pub mpris_art_url: Option<String>,
    /// Length in microseconds.
    pub mpris_length: Option<u64>,
    pub xesam_title: Option<String>,
    pub xesam_album: Option<String>,
    pub xesam_artist: Option<String>,
    pub xesam_album_artist: Option<String>,
    pub xesam_url: Option<String>,
    /// Example value: 2018-06-28T00:00:00
    pub xesam_content_created: Option<String>,
    /// Raw metadata values.
    pub raw: HashMap<String, String>
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

    /// Get current player positions, in microseconds.
    pub fn get_position() -> Result<HashMap<String, u64>> {
        let output = run_command("status -a -f {{playerName}};-;{{position}}")?;
        let mut m = HashMap::new();
        for line in output.lines() {
            let Some((name, pos)) = line.split_once(";-;") else {
                continue;
            };
            m.insert(name.to_owned(), pos.parse()?);
        }
        Ok(m)
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

        match status.as_str().trim() {
            "Playing" => Ok(TrackStatus::Playing),
            "Paused" => Ok(TrackStatus::Paused),
            _ => Ok(TrackStatus::Stopped),
        }
    }

    /// Get metadata information for all active players.
    ///
    /// ```
    /// let metadata = Playerctl::metadata().unwrap();
    ///
    /// println!("Title: {}", metadata.values().first().unwrap().xesam_title.unwrap());
    /// ```
    pub fn metadata() -> Result<HashMap<String, PlayerMetadata>> {
        let mut data: HashMap<_, PlayerMetadata> = HashMap::new();

        let all = run_command("metadata -a")?;
        for line in all.lines() {
            let Some((player,b)) = line.split_once(' ') else {
                continue;
            };
            let b = b.trim_ascii_start();
            let Some((key,val)) = b.split_once(' ') else {
                continue;
            };
            let val = val.trim_ascii_start();
            let map = data.entry(player.to_owned()).or_default();
            match key {
                "mpris:artUrl" => map.mpris_art_url = Some(urlencoding::decode(&val)?.into_owned()),
                "mpris:length" => map.mpris_length = Some(val.parse()?),
                "mpris:trackid" => map.mpris_trackid = Some(val.to_owned()),
                "xesam:album" => map.xesam_album = Some(val.to_owned()),
                "xesam:albumArtist" => map.xesam_album_artist = Some(val.to_owned()),
                "xesam:artist" => map.xesam_artist = Some(val.to_owned()),
                "xesam:contentCreated" => map.xesam_content_created = Some(val.to_owned()),
                "xesam:title" => map.xesam_title = Some(val.to_owned()),
                "xesam:url" => map.xesam_url = Some(urlencoding::decode(&val)?.into_owned()),
                _ => {}
            }
            map.raw.insert(key.to_owned(), val.to_owned());
        }
        Ok(data)
    }
}

/// Run a playerctl command.
///
/// ```
/// Playerctl::play().expect("Failed to run command");
/// ```
fn run_command(command: &str) -> Result<String> {
    let args = command.split_whitespace();

    let output = Command::new("playerctl").args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        Err(PlayerctlError::CommandError(format!(
            "Command failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}
