# Playerctl Rust Wrapper

Rust wrapper for [`playerctl`](https://github.com/altdesktop/playerctl), allowing the control of the active player and current player track.

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

    println!("{metadata:#?}");
}
```

### Output

```
{
    "mpv": PlayerMetadata {
        mpris_trackid: Some(
            "'/69'",
        ),
        mpris_art_url: Some(
            "file:///home/user/Music/cover.jpg",
        ),
        mpris_length: Some(
            160680000,
        ),
        xesam_title: None,
        xesam_album: None,
        xesam_artist: None,
        xesam_album_artist: None,
        xesam_url: Some(
            "file:///home/user/Music/Ilmari Hakkola - Bad Piggies Theme.opus",
        ),
        xesam_content_created: Some(
            "2020-04-23T00:00:00",
        ),
        raw: {
            "mpris:trackid": "'/71'",
            "mpris:artUrl": "file:///home/user/Music/cover.jpg",
            "mpris:length": "160680000",
            "xesam:contentCreated": "2020-04-23T00:00:00",
            "xesam:url": "file:///home/user/Music/Ilmari%20Hakkola%20-%20Bad%20Piggies%20Theme.opus",
        },
    },
    "firefox": PlayerMetadata {
        mpris_trackid: Some(
            "'/org/mpris/MediaPlayer2/firefox'",
        ),
        mpris_art_url: Some(
            "file:///home/user/.mozilla/firefox/firefox-mpris/6787_7.png",
        ),
        mpris_length: Some(
            862000000,
        ),
        xesam_title: None,
        xesam_album: None,
        xesam_artist: None,
        xesam_album_artist: None,
        xesam_url: Some(
            "https://www.youtube.com/watch?v=I9nK9FggzIg",
        ),
        xesam_content_created: None,
        raw: {
            "mpris:length": "862000000",
            "xesam:url": "https://www.youtube.com/watch?v=I9nK9FggzIg",
            "mpris:trackid": "'/org/mpris/MediaPlayer2/firefox'",
            "mpris:artUrl": "file:///home/user/.mozilla/firefox/firefox-mpris/6787_7.png",
        },
    },
}
```

# License

MIT License

Copyright (c) 2025 Henrique Bacelar
Copyright (c) 2025 FliegendeWurst

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
