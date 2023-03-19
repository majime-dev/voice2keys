use crossterm::event::{read, Event};

pub fn pause(message: &str) {
    log::info!("{}", message);

    loop {
        if let Event::Key(_) = read().unwrap() {
            return;
        }
    }
}
