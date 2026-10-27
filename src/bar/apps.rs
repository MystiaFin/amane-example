use amane::{Center, Image, Pointer, Rectangle};

use crate::{files, launcher};

pub fn view() -> Rectangle {
    let icon = Rectangle::new()
        .width(16.0)
        .height(16.0)
        .fill(Image::contain(files::find("assets/linux.svg")));

    Rectangle::new()
        .width(32.0)
        .height(24.0)
        .fill("#cba6f7")
        .radius(6.0)
        .cursor(Pointer)
        .align_child(Center, Center)
        .on_click(|_| launcher::toggle())
        .child(icon)
}
