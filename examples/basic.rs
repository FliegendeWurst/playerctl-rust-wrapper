use playerctl_rust_wrapper::Playerctl;

fn main() {
    // Command the player to play
    Playerctl::play().unwrap_or_default();

    // Command the player to pause
    Playerctl::pause().unwrap_or_default();

    // Command the player to toggle between play/pause
    Playerctl::play_pause().unwrap_or_default();

    // Command the player to seek forward/backward OFFSET in seconds
    Playerctl::position(10.).unwrap_or_default();

    // Get metadata information for the current track
    let metadata = Playerctl::metadata().unwrap_or_default();

    println!("{metadata:#?}");
}
