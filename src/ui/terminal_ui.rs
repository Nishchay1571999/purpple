use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

pub fn create_terminal_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 10);
    let button = gtk4::Button::with_label("Show all everything going on in Terminal");
    vbox.append(&button);

    vbox
}
