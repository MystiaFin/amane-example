mod bar;

use amane::App;

fn main() {
    App::new().window_per_monitor(bar::view).run();
}
