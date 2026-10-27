use amane::{Color, Column, Notifications, Parent, ScrollArea, Service, Text, Widget, children};

use crate::{notifications, panel};

pub fn view() -> Column {
    let list = Notifications::read();

    let header = panel::header("Notifications", "clear", |_| Notifications::clear());

    let mut cards: Vec<Box<dyn Widget>> = Vec::new();

    // the list is oldest first, the newest should be on top
    for notification in list.list().iter().rev() {
        cards.push(Box::new(notifications::card(notification)));
    }

    if cards.is_empty() {
        let message = Text::new("nothing new")
            .size(14.0)
            .color(Color::from("#a6adc8"));

        cards.push(Box::new(message));
    }

    let cards = ScrollArea::new("notifications", Column::new(cards).width(Parent).gap(8.0));

    Column::new(children![header, cards])
        .width(Parent)
        .height(Parent)
        .gap(12.0)
}
