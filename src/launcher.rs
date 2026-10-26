mod entry;

use std::time::Duration;

use amane::{
    Apps, Color, Column, DesktopApp, Keyboard, Layer, LayerWindow, Parent, Rectangle, ScrollArea,
    Service, TextInput, Widget, children,
};

pub struct Launcher {
    pub open: bool,
    pub query: String,
}

impl Service for Launcher {
    fn new() -> Self {
        Self {
            open: false,
            query: String::new(),
        }
    }

    // nothing changes on its own, only through ipc and input
    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

pub fn view() -> LayerWindow {
    let launcher = Launcher::read();

    let search = TextInput::new("launcher")
        .placeholder("search apps")
        .size(16.0)
        .color(Color::WHITE)
        .focused()
        .on_change(|text| Launcher::write().query = text);

    let search_box = Rectangle::new()
        .width(Parent)
        .height(40.0)
        .fill("#313244")
        .radius(8.0)
        .padding(10.0)
        .child(search);

    let list = ScrollArea::new("launcher", Column::new(entries(&launcher.query)).gap(2.0));

    let content = Column::new(children![search_box, list])
        .width(Parent)
        .height(Parent)
        .gap(12.0);

    // exclusive, so typing goes to the search box and not the window behind
    LayerWindow::new()
        .width(420.0)
        .height(460.0)
        .layer(Layer::Overlay)
        .keyboard(Keyboard::Exclusive)
        .namespace("launcher")
        .visible(launcher.open)
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

pub fn toggle() {
    let is_open = Launcher::read().open;

    if is_open {
        close();
    } else {
        Launcher::write().open = true;
    }
}

pub fn close() {
    // the next open starts with an empty search
    TextInput::set_text("launcher", "");

    let mut launcher = Launcher::write();

    launcher.open = false;
    launcher.query.clear();
}

fn entries(query: &str) -> Vec<Box<dyn Widget>> {
    let apps = Apps::read();

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    for app in apps.list() {
        if !matches(app, query) {
            continue;
        }

        rows.push(Box::new(entry::view(app)));
    }

    rows
}

fn matches(app: &DesktopApp, query: &str) -> bool {
    let name = app.name().to_lowercase();

    let query = query.to_lowercase();

    name.contains(&query)
}
