mod wifi;

use std::time::Duration;

use amane::{
    Button, Center, Color, Column, Horizontal, Key, Keyboard, Layer, LayerWindow, Margin, Network,
    Parent, Pointer, Rectangle, Row, Service, SpaceBetween, Text, Vertical, children,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Panel {
    Wifi,
}

// which panel is open, only one at a time
pub struct Panels {
    pub open: Option<Panel>,
}

impl Service for Panels {
    fn new() -> Self {
        Self { open: None }
    }

    // nothing changes on its own, only through clicks
    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

pub fn view() -> LayerWindow {
    let panels = Panels::read();

    let content = match panels.open {
        Some(Panel::Wifi) => wifi::view(),
        None => Column::new(Vec::new()),
    };

    let margin = Margin {
        top: 8,
        right: 8,
        bottom: 0,
        left: 0,
    };

    // on demand, so the wifi password box can take keys after a click
    LayerWindow::new()
        .width(340.0)
        .height(420.0)
        .anchor_vertical(Vertical::Top)
        .anchor_horizontal(Horizontal::Right)
        .margin(margin)
        .layer(Layer::Top)
        .keyboard(Keyboard::OnDemand)
        .namespace("panel")
        .visible(panels.open.is_some())
        .on_key(key_pressed)
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill("#1e1e2e")
                .radius(12.0)
                .border(1.0, "#45475a")
                .padding(12.0)
                .child(content),
        )
}

// clicking the same bar button again closes its panel
pub fn toggle(panel: Panel) {
    let open = Panels::read().open;

    if open == Some(panel) {
        Panels::write().open = None;

        return;
    }

    // wifi networks show up a few seconds after a scan
    if panel == Panel::Wifi {
        Network::scan();
    }

    Panels::write().open = Some(panel);
}

fn key_pressed(key: Key) {
    if key == Key::Escape {
        Panels::write().open = None;
    }
}

// a title on the left and one small button on the right
pub fn header(title: &str, button: &str, on_click: impl Fn(Button) + 'static) -> Row {
    let title = Text::new(title).size(16.0).color(Color::WHITE);

    let button = Rectangle::new()
        .width(72.0)
        .height(28.0)
        .fill("#313244")
        .radius(6.0)
        .cursor(Pointer)
        .align_child(Center, Center)
        .on_click(on_click)
        .child(Text::new(button).size(13.0).color(Color::WHITE));

    Row::new(children![title, button])
        .width(Parent)
        .justify(SpaceBetween)
        .align(Center)
}

// one line in a panel list, with a name on the left and its state on the right
pub fn item(name: &str, state: &str) -> Rectangle {
    let name = Text::new(name).size(14.0).color(Color::WHITE);
    let state = Text::new(state).size(12.0).color(Color::WHITE);

    let line = Row::new(children![name, state])
        .width(Parent)
        .justify(SpaceBetween)
        .align(Center);

    Rectangle::new()
        .width(Parent)
        .height(36.0)
        .fill(Color::rgba(49, 50, 68, 120))
        .radius(8.0)
        .padding(10.0)
        .cursor(Pointer)
        .child(line)
}
