use amane::{Center, Color, DesktopApp, Image, Parent, Pointer, Rectangle, Row, Text, children};

use crate::launcher;

pub fn view(app: &DesktopApp) -> Rectangle {
    let launched = app.clone();

    let icon = Rectangle::new().width(24.0).height(24.0);

    // apps without an icon keep an empty square, so the names still line up
    let icon = match app.icon_path() {
        Some(path) => icon.fill(Image::contain(path)),
        None => icon,
    };

    let name = Text::new(app.name()).size(15.0).color(Color::WHITE).elide();

    Rectangle::new()
        .width(Parent)
        .height(40.0)
        .fill(Color::rgba(49, 50, 68, 120))
        .radius(8.0)
        .padding(8.0)
        .cursor(Pointer)
        .on_click(move |_| {
            launched.launch();

            launcher::close();
        })
        .child(Row::new(children![icon, name]).gap(10.0).align(Center))
}
