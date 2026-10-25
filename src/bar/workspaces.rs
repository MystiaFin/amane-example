use amane::{
    Center, Color, Monitor, Pointer, Rectangle, Row, Service, Text, Widget, Workspace, Workspaces,
};

pub fn view(monitor: &Monitor) -> Row {
    let workspaces = Workspaces::read();

    let mut buttons: Vec<Box<dyn Widget>> = Vec::new();

    for workspace in workspaces.list() {
        // every bar only shows the workspaces of its own monitor
        if workspace.output() != Some(monitor.name.as_str()) {
            continue;
        }

        buttons.push(Box::new(button(workspace)));
    }

    Row::new(buttons).gap(4.0)
}

fn button(workspace: &Workspace) -> Rectangle {
    let fill = if workspace.focused() {
        "#89b4fa"
    } else if workspace.urgent() {
        "#f38ba8"
    } else {
        "#313244"
    };

    let id = workspace.id();

    let label = workspace.index().to_string();

    Rectangle::new()
        .width(24.0)
        .height(24.0)
        .fill(fill)
        .radius(6.0)
        .cursor(Pointer)
        .align_child(Center, Center)
        .on_click(move |_| Workspaces::focus(id))
        .child(Text::new(label).size(13.0).color(Color::WHITE))
}
