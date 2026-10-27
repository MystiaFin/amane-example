use amane::{
    Color, Column, Horizontal, Layer, LayerWindow, Margin, Notification, Notifications, Parent,
    Pointer, Rectangle, Service, Text, Urgency, Vertical, Weight, Widget, children,
};

use crate::panel::Panels;

const WIDTH: f32 = 360.0;
const CARD_HEIGHT: f32 = 64.0;
const GAP: f32 = 8.0;

// older ones wait their turn until newer ones are dismissed
const MAX_SHOWN: usize = 4;

pub fn view() -> LayerWindow {
    let notifications = Notifications::read();

    let mut cards: Vec<Box<dyn Widget>> = Vec::new();

    // the list is oldest first, the newest should be on top
    for notification in notifications.list().iter().rev() {
        if cards.len() == MAX_SHOWN {
            break;
        }

        cards.push(Box::new(card(notification)));
    }

    /*
     * the window is only as tall as its cards, so the empty
     * space below them never swallows clicks. it can't be
     * 0 pixels tall though, even while hidden
     */
    let count = cards.len() as f32;
    let height = f32::max(count * (CARD_HEIGHT + GAP), 1.0);

    // panels open in the same corner, the popups wait until they close
    let panel_open = Panels::read().open.is_some();

    let margin = Margin {
        top: 8,
        right: 8,
        bottom: 0,
        left: 0,
    };

    LayerWindow::new()
        .width(WIDTH)
        .height(height)
        .anchor_vertical(Vertical::Top)
        .anchor_horizontal(Horizontal::Right)
        .margin(margin)
        .layer(Layer::Overlay)
        .namespace("notifications")
        .visible(!cards.is_empty() && !panel_open)
        .child(Column::new(cards).gap(GAP))
}

// clicking runs the notification's default action, or dismisses it
pub fn card(notification: &Notification) -> Rectangle {
    let id = notification.id();

    // critical ones get a red title, so they stand out
    let title_color = match notification.urgency() {
        Urgency::Critical => Color::from("#f38ba8"),
        _ => Color::from("#cdd6f4"),
    };

    let summary = Text::new(notification.summary())
        .size(14.0)
        .weight(Weight::SemiBold)
        .color(title_color)
        .elide();

    let body = Text::new(notification.body())
        .size(13.0)
        .color(Color::from("#a6adc8"))
        .elide();

    Rectangle::new()
        .width(Parent)
        .height(CARD_HEIGHT)
        .fill(Color::rgb(30, 30, 46))
        .radius(10.0)
        .padding(12.0)
        .cursor(Pointer)
        .on_click(move |_| Notifications::click(id))
        .child(Column::new(children![summary, body]).gap(4.0))
}
