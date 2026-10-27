use amane::{
    Audio, Battery, Button, Center, Color, End, Link, Network, Parent, Pointer, Rectangle, Row,
    Scroll, Service, Text, Widget,
};

use crate::panel::{self, Panel};

pub fn view() -> Row {
    let mut items: Vec<Box<dyn Widget>> = Vec::new();

    items.push(Box::new(volume()));
    items.push(Box::new(wifi()));

    // desktops have no battery, so there's nothing to show
    if Battery::read().present() {
        items.push(Box::new(battery()));
    }

    Row::new(items).gap(16.0).align(Center)
}

// click to mute, scroll to change the volume
fn volume() -> Rectangle {
    let audio = Audio::read();

    let label = if audio.muted() {
        String::from("muted")
    } else {
        format!("vol {}%", audio.volume())
    };

    button(label, 72.0)
        .cursor(Pointer)
        .on_click(volume_clicked)
        .on_scroll(volume_scrolled)
}

fn volume_clicked(_: Button) {
    Audio::toggle_mute();
}

fn volume_scrolled(scroll: Scroll) {
    let volume = i32::from(Audio::read().volume());

    // scrolling up is a negative y, and up should mean louder
    let step = if scroll.y < 0.0 { 5 } else { -5 };

    let changed = (volume + step).clamp(0, 100);

    Audio::set_volume(changed as u8);
}

fn wifi() -> Rectangle {
    let network = Network::read();

    let label = match network.link() {
        Link::Wifi => format!("{} {}%", network.ssid(), network.strength()),
        Link::Wired => String::from("wired"),
        Link::Other => String::from("online"),
        Link::Offline => String::from("offline"),
    };

    button(label, 140.0)
        .cursor(Pointer)
        .on_click(|_| panel::toggle(Panel::Wifi))
}

fn battery() -> Rectangle {
    let battery = Battery::read();

    let label = if battery.charging() {
        format!("bat {}% +", battery.percent())
    } else {
        format!("bat {}%", battery.percent())
    };

    button(label, 72.0)
}

// fixed widths, so the bar doesn't jump around when a label changes
fn button(label: String, width: f32) -> Rectangle {
    let text = Text::new(label).size(14.0).color(Color::WHITE).elide();

    Rectangle::new()
        .width(width)
        .height(Parent)
        .align_child(End, Center)
        .child(text)
}
