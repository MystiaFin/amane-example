use amane::{Center, Color, Column, End, Lock, Parent, Pointer, Rectangle, Start, Text, children};

use crate::launcher;

// the narrow column on the left of the menu
pub fn view() -> Rectangle {
    let buttons = Column::new(children![
        button("sleep", sleep),
        button("lock", lock),
        button("log out", log_out),
        button("shut down", shut_down),
    ])
    .width(Parent)
    .height(Parent)
    .justify(End)
    .gap(4.0);

    Rectangle::new()
        .width(112.0)
        .height(Parent)
        .fill("#181825")
        .radius(8.0)
        .padding(8.0)
        .child(buttons)
}

// the menu closes first, so it isn't still open after waking up or unlocking
fn button(label: &str, action: fn()) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(36.0)
        .fill("#313244")
        .radius(6.0)
        .padding(10.0)
        .cursor(Pointer)
        .align_child(Start, Center)
        .on_click(move |_| {
            launcher::close();

            action();
        })
        .child(Text::new(label).size(14.0).color(Color::from("#cdd6f4")))
}

fn sleep() {
    amane::spawn("systemctl suspend");
}

fn lock() {
    Lock::start();
}

// ends only this login session, the one the shell runs in
fn log_out() {
    amane::spawn("loginctl terminate-session \"$XDG_SESSION_ID\"");
}

fn shut_down() {
    amane::spawn("systemctl poweroff");
}
