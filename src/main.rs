mod bar;
mod clock;
mod launcher;

use amane::App;

fn main() {
    App::new()
        .ipc("launcher", toggle_launcher)
        .window_per_monitor(bar::view)
        .window(launcher::view)
        .run();
}

// bind it to a key in your compositor: amane ipc call launcher
fn toggle_launcher(_: &[String]) -> String {
    launcher::toggle();

    String::from("toggled the launcher")
}
