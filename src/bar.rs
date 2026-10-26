mod clock;
mod status;
mod workspaces;

use amane::{
    Center, Full, Layer, LayerWindow, Monitor, Padding, Parent, Rectangle, Row, SpaceBetween,
    Vertical, Zone, children,
};

const HEIGHT: f32 = 32.0;

pub fn view(monitor: &Monitor) -> LayerWindow {
    let padding = Padding {
        top: 0.0,
        right: 8.0,
        bottom: 0.0,
        left: 8.0,
    };

    let sections = Row::new(children![
        workspaces::view(monitor),
        clock::view(),
        status::view(),
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
