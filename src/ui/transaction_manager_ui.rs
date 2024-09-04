use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

pub fn create_transaction_manager_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 10);
    let button = gtk4::Button::with_label("Create Transaction");
    vbox.append(&button);

    vbox
}
