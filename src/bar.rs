use amane::{
    Center, Color, Full, Layer, LayerWindow, Monitor, Padding, Parent, Rectangle, Row,
    SpaceBetween, Text, Vertical, Zone, children,
};

const HEIGHT: f32 = 32.0;

pub fn view(_monitor: &Monitor) -> LayerWindow {
    let padding = Padding {
        top: 0.0,
        right: 8.0,
        bottom: 0.0,
        left: 8.0,
    };

    let sections = Row::new(children![
        Text::new("left").color(Color::WHITE),
        Text::new("center").color(Color::WHITE),
        Text::new("right").color(Color::WHITE),
    ])
    .width(Parent)
    .height(Parent)
    .justify(SpaceBetween)
    .align(Center);

    // reserve keeps maximized windows from sliding under the bar
    LayerWindow::new()
        .width(Full)
        .height(HEIGHT)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .space(Zone::Reserve)
        .namespace("bar")
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill("#1e1e2e")
                .padding(padding)
                .child(sections),
        )
}
