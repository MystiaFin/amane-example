use amane::{Center, Color, Pointer, Rectangle, Text};

use crate::launcher;

pub fn view() -> Rectangle {
    Rectangle::new()
        .width(52.0)
        .height(24.0)
        .fill("#cba6f7")
        .radius(6.0)
        .cursor(Pointer)
        .align_child(Center, Center)
        .on_click(|_| launcher::toggle())
        .child(Text::new("apps").size(13.0).color(Color::WHITE))
}
