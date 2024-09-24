use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation};

pub fn create_action_buttons_box() -> Box {
    let create_button = Button::with_label("Create Account");
    let import_button = Button::with_label("Import Account");
    let delete_button = Button::with_label("Delete Account");
    let send_button = Button::with_label("Send Tokens");

    let action_box = Box::new(Orientation::Horizontal, 10);
    
    action_box.append(&create_button);
    action_box.append(&import_button);
    action_box.append(&delete_button);
    action_box.append(&send_button);
    
    action_box.set_margin_top(10);
    action_box.set_margin_start(10);
    action_box.set_margin_end(10);
    action_box.set_margin_bottom(10);

    action_box
}
