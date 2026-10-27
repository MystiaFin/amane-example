mod bar;
mod clock;
mod launcher;
mod lock_screen;
mod notifications;
mod panel;
mod wallpaper;

use amane::{App, Lock};

fn main() {
    App::new()
        .ipc("launcher", toggle_launcher)
        .ipc("lock", lock)
        .window_per_monitor(wallpaper::view)
        .window_per_monitor(bar::view)
        .window(launcher::view)
        .window(notifications::view)
        .window(panel::view)
        .lock(lock_screen::view)
        .run();
}

// bind it to a key in your compositor: amane ipc call launcher
fn toggle_launcher(_: &[String]) -> String {
    launcher::toggle();

    String::from("toggled the launcher")
}

// for a keybind or an idle daemon: amane ipc call lock
fn lock(_: &[String]) -> String {
    Lock::start();

    String::from("locked")
}
