use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};


pub fn create_account_manager_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 10);
    let button = gtk4::Button::with_label("Create Account");
    vbox.append(&button);

    vbox
}
