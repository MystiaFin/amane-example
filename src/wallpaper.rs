use std::path::{Path, PathBuf};

use amane::{Full, Image, Layer, LayerWindow, Monitor, Parent, Rectangle, Zone};

// relative to this file, or a full path like "/home/you/Pictures/wall.jpg"
const WALLPAPER: &str = "wallpaper-example/wallpaper_2.jpg";

pub fn view(_monitor: &Monitor) -> LayerWindow {
    let image = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill(Image::cover(find_wallpaper()));

    // ignore, so it also covers the space the bar reserves
    LayerWindow::new()
        .width(Full)
        .height(Full)
        .layer(Layer::Background)
        .space(Zone::Ignore)
        .namespace("wallpaper")
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill("#1e1e2e")
                .child(image),
        )
}

/*
 * the shell can be started from any folder, so a plain relative
 * path would break. file!() is this file's full path from when
 * amane built it, so the image is looked up next to it instead
 */
fn find_wallpaper() -> PathBuf {
    let this_file = Path::new(file!());

    let folder = this_file.parent().unwrap_or(Path::new("."));

    // joining a full path just gives that full path back
    folder.join(WALLPAPER)
}
