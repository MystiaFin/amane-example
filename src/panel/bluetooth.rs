use amane::{
    Bluetooth, BluetoothDevice, Color, Column, Parent, Rectangle, ScrollArea, Service, Text,
    Widget, children,
};

use crate::panel;

pub fn view() -> Column {
    let bluetooth = Bluetooth::read();

    // no adapter, or bluez isn't running
    if !bluetooth.available() {
        let message = Text::new("no bluetooth adapter")
            .size(14.0)
            .color(Color::from("#a6adc8"));

        return Column::new(children![message]);
    }

    let powered = bluetooth.powered();

    let button = if powered { "turn off" } else { "turn on" };

    let header = panel::header("Bluetooth", button, move |_| {
        Bluetooth::set_powered(!powered)
    });

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    for device in bluetooth.devices() {
        rows.push(Box::new(device_row(device)));
    }

    let list = ScrollArea::new("bluetooth", Column::new(rows).width(Parent).gap(4.0));

    Column::new(children![header, list])
        .width(Parent)
        .height(Parent)
        .gap(12.0)
}

fn device_row(device: &BluetoothDevice) -> Rectangle {
    let path = String::from(device.path());

    let connected = device.connected();
    let paired = device.paired();

    let state = if connected {
        "connected"
    } else if paired {
        "paired"
    } else {
        "new"
    };

    panel::item(device.name(), state).on_click(move |_| clicked(&path, connected, paired))
}

// pairing a new device also connects it
fn clicked(path: &str, connected: bool, paired: bool) {
    if connected {
        Bluetooth::disconnect(path);

        return;
    }

    if paired {
        Bluetooth::connect(path);

        return;
    }

    Bluetooth::pair(path);
}
