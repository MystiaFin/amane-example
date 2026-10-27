use amane::{Full, Image, Layer, LayerWindow, Monitor, Parent, Rectangle, Zone};

/*
 * relative to the folder amane was started from, so run amane
 * from this project, or put a full path here
 */
const WALLPAPER: &str = "wallpaper-example/image.jpg";

pub fn view(_monitor: &Monitor) -> LayerWindow {
    let image = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill(Image::cover(WALLPAPER));

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
