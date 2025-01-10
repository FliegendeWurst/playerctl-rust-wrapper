# Playerctl Rust Wrapper

A Rust implementation for a [`playerctl`](https://github.com/altdesktop/playerctl)
wrapper, allowing the control of the  active player and current player track.

## Example

```rust
use playerctl_wrapper::Playerctl;

fn main() {
    // Command the player to play
    Playerctl::play().unwrap();

    // Command the player to pause
    Playerctl::pause().unwrap();

    // Command the player to toggle between play/pause
    Playerctl::play_pause().unwrap();

    // Command the player to seek forward/backward OFFSET in seconds
    Playerctl::position(10.).unwrap();

    // Get metadata information for the current track
    let metadata = Playerctl::metadata().unwrap();

    println!(
        "Title: {}\nAlbum: {}\nArtist: {}\nURL: {}\nLength: {}",
        metadata.title, metadata.album, metadata.artist, metadata.url, metadata.length
    )
}
```

# Author

Henrique BACELAR