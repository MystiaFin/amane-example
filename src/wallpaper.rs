use amane::{Full, Image, Layer, LayerWindow, Monitor, Parent, Rectangle, Zone};

use crate::files;

// relative to the project folder, or a full path like "/home/you/Pictures/wall.jpg"
const WALLPAPER: &str = "src/wallpaper-example/wallpaper_2.jpg";

pub fn view(_monitor: &Monitor) -> LayerWindow {
    let image = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill(Image::cover(files::find(WALLPAPER)));

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
