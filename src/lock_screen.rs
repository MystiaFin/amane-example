use amane::{
    Center, Color, Column, LayerWindow, Lock, Monitor, Parent, Rectangle, Service, Text,
    TextInput, Weight, children,
};

use crate::clock::Clock;

// the compositor stretches lock screens over the whole monitor, so this size is never used
pub fn view(_monitor: &Monitor) -> LayerWindow {
    let lock = Lock::read();
    let clock = Clock::read();

    let status = if lock.checking() {
        "checking..."
    } else if lock.failed() {
        "wrong password, try again"
    } else {
        "type your password and press enter"
    };

    let time = Text::new(&clock.time)
        .size(72.0)
        .weight(Weight::Bold)
        .color(Color::WHITE);

    let date = Text::new(&clock.date).size(18.0).color(Color::WHITE);

    let password = TextInput::new("password")
        .placeholder("password")
        .password()
        .focused()
        .size(16.0)
        .color(Color::WHITE)
        .on_submit(submitted);

    let password_box = Rectangle::new()
        .width(300.0)
        .height(44.0)
        .fill("#313244")
        .radius(8.0)
        .padding(12.0)
        .child(password);

    let content = Column::new(children![
        time,
        date,
        password_box,
        Text::new(status).size(14.0).color(Color::WHITE),
    ])
    .gap(16.0)
    .align(Center);

    LayerWindow::new().width(1.0).height(1.0).child(
        Rectangle::new()
            .width(Parent)
            .height(Parent)
            .fill(Color::rgb(17, 17, 27))
            .align_child(Center, Center)
            .child(content),
    )
}

// a wrong password shouldn't stay in the box for the next try
fn submitted(password: String) {
    TextInput::set_text("password", "");

    Lock::unlock(&password);
}
