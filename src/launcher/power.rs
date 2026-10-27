use std::time::Duration;

use amane::{
    Center, Color, Column, End, Lock, Parent, Pointer, Rectangle, Service, Start, Text, children,
};

use crate::launcher;

// log out and shut down lose unsaved work, so they wait for a second click
pub struct Confirm {
    pub waiting: Option<&'static str>,
}

impl Service for Confirm {
    fn new() -> Self {
        Self { waiting: None }
    }

    // nothing changes on its own, only through clicks
    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

// the narrow column on the left of the menu
pub fn view() -> Rectangle {
    let waiting = Confirm::read().waiting;

    let buttons = Column::new(children![
        button("sleep", sleep),
        button("lock", lock),
        confirmed_button("log out", log_out, waiting),
        confirmed_button("shut down", shut_down, waiting),
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

// a closed menu forgets a half-confirmed click
pub fn cancel() {
    Confirm::write().waiting = None;
}

fn button(label: &str, action: fn()) -> Rectangle {
    plain_button(label, "#313244", "#cdd6f4").on_click(move |_| run(action))
}

// the first click only turns the button red, the second one runs it
fn confirmed_button(label: &'static str, action: fn(), waiting: Option<&str>) -> Rectangle {
    if waiting == Some(label) {
        return plain_button("sure?", "#f38ba8", "#1e1e2e").on_click(move |_| run(action));
    }

    plain_button(label, "#313244", "#cdd6f4")
        .on_click(move |_| Confirm::write().waiting = Some(label))
}

// the menu closes first, so it isn't still open after waking up or unlocking
fn run(action: fn()) {
    launcher::close();

    action();
}

fn plain_button(label: &str, fill: &str, text: &str) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(36.0)
        .fill(fill)
        .radius(6.0)
        .padding(10.0)
        .cursor(Pointer)
        .align_child(Start, Center)
        .child(Text::new(label).size(14.0).color(Color::from(text)))
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
