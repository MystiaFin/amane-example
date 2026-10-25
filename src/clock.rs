use amane::Service;

/*
 * date already knows the timezone, so the shell
 * doesn't need a time crate just for this
 */
pub struct Clock {
    pub time: String,
    pub date: String,
}

impl Service for Clock {
    fn new() -> Self {
        Self {
            time: read_time(),
            date: read_date(),
        }
    }

    fn update(&mut self) -> bool {
        let time = read_time();
        let date = read_date();

        let changed = time != self.time || date != self.date;

        self.time = time;
        self.date = date;

        changed
    }
}

fn read_time() -> String {
    amane::output("date +%H:%M")
}

fn read_date() -> String {
    amane::output("date '+%a %d %b'")
}
