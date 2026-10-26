use amane::{
    Audio, Battery, Button, Center, Color, End, Link, Network, Parent, Pointer, Rectangle, Row,
    Scroll, Service, Text, Widget,
};

pub fn view() -> Row {
    let mut items: Vec<Box<dyn Widget>> = Vec::new();

    items.push(Box::new(volume()));
    items.push(Box::new(network()));

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

    Rectangle::new()
        .width(72.0)
        .height(Parent)
        .cursor(Pointer)
        .align_child(End, Center)
        .on_click(volume_clicked)
        .on_scroll(volume_scrolled)
        .child(Text::new(label).size(14.0).color(Color::WHITE))
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

fn network() -> Text {
    let network = Network::read();

    let label = match network.link() {
        Link::Wifi => format!("{} {}%", network.ssid(), network.strength()),
        Link::Wired => String::from("wired"),
        Link::Other => String::from("online"),
        Link::Offline => String::from("offline"),
    };

    Text::new(label).size(14.0).color(Color::WHITE)
}

fn battery() -> Text {
    let battery = Battery::read();

    let label = if battery.charging() {
        format!("bat {}% +", battery.percent())
    } else {
        format!("bat {}%", battery.percent())
    };

    Text::new(label).size(14.0).color(Color::WHITE)
}
