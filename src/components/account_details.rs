use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

pub fn create_account_details_box() -> Box {
    let details_box = Box::new(Orientation::Vertical, 10);
    
    let name_label = Label::new(Some("Account Name:"));
    let address_label = Label::new(Some("Address:"));
    let balance_label = Label::new(Some("Balance:"));
    
    details_box.append(&name_label);
    details_box.append(&address_label);
    details_box.append(&balance_label);
    
    details_box.set_margin_top(10);
    details_box.set_margin_start(10);
    details_box.set_margin_end(10);
    details_box.set_margin_bottom(10);
    
    details_box
}
