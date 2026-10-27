use std::time::Duration;

use amane::{
    AccessPoint, Color, Column, Network, Parent, Rectangle, ScrollArea, Service, Text, TextInput,
    Widget, children,
};

use crate::panel;

// the secured network that's waiting for its password
pub struct WifiPassword {
    pub ssid: Option<String>,
}

impl Service for WifiPassword {
    fn new() -> Self {
        Self { ssid: None }
    }

    // nothing changes on its own, only through clicks
    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

pub fn view() -> Column {
    let network = Network::read();
    let password = WifiPassword::read();

    let enabled = network.wifi_enabled();

    let button = if enabled { "turn off" } else { "turn on" };

    let header = panel::header("Wi-Fi", button, move |_| Network::set_wifi(!enabled));

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    if let Some(ssid) = &password.ssid {
        rows.push(Box::new(password_box(ssid)));
    }

    for access_point in network.access_points() {
        // hidden networks have no name to show or join by
        if access_point.ssid().is_empty() {
            continue;
        }

        rows.push(Box::new(access_point_row(access_point)));
    }

    let list = ScrollArea::new("wifi", Column::new(rows).width(Parent).gap(4.0));

    Column::new(children![header, list])
        .width(Parent)
        .height(Parent)
        .gap(12.0)
}

fn access_point_row(access_point: &AccessPoint) -> Rectangle {
    let ssid = String::from(access_point.ssid());

    let active = access_point.active();

    // saved networks already have their password in networkmanager
    let needs_password = access_point.secured() && !access_point.saved();

    let state = if active {
        String::from("connected")
    } else {
        format!("{}%", access_point.strength())
    };

    panel::item(&ssid, &state).on_click(move |_| clicked(&ssid, active, needs_password))
}

fn clicked(ssid: &str, active: bool, needs_password: bool) {
    if active {
        Network::disconnect();

        return;
    }

    if needs_password {
        WifiPassword::write().ssid = Some(String::from(ssid));

        return;
    }

    Network::connect(ssid, None);
}

fn password_box(ssid: &str) -> Column {
    let label = Text::new(format!("password for {ssid}"))
        .size(13.0)
        .color(Color::WHITE);

    let input = TextInput::new("wifi-password")
        .placeholder("password")
        .password()
        .focused()
        .size(14.0)
        .color(Color::WHITE)
        .on_submit(submitted);

    let field = Rectangle::new()
        .width(Parent)
        .height(36.0)
        .fill("#313244")
        .radius(8.0)
        .padding(10.0)
        .child(input);

    Column::new(children![label, field]).width(Parent).gap(6.0)
}

fn submitted(password: String) {
    let ssid = WifiPassword::write().ssid.take();

    TextInput::set_text("wifi-password", "");

    let Some(ssid) = ssid else {
        return;
    };

    Network::connect(&ssid, Some(&password));
}
