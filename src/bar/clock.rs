use amane::{Color, Service, Text};

use crate::clock::Clock;

pub fn view() -> Text {
    let clock = Clock::read();

    let label = format!("{}  {}", clock.date, clock.time);

    Text::new(label).size(14.0).color(Color::from("#cdd6f4"))
}
