use gtk4::prelude::*;
use gtk4::{Box, Button, ComboBoxText, Label, Orientation, SearchEntry};

pub fn create_block_explorer_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 10);

    // Create the top navbar
    let navbar = create_top_navbar();
    vbox.append(&navbar);

    let search_bar = SearchEntry::new();
    search_bar.set_placeholder_text(Some("Search blocks, transactions, or accounts..."));
    search_bar.set_margin_top(10);
    search_bar.set_margin_start(10);
    search_bar.set_margin_end(10);
    search_bar.set_margin_bottom(10);
    let details_box = Box::new(Orientation::Vertical, 10);
    let block_details_label = Label::new(Some("Block Details:"));
    let transaction_details_label = Label::new(Some("Transaction Details:"));
    details_box.append(&block_details_label);
    details_box.append(&transaction_details_label);
    vbox.append(&details_box);

    vbox
}

fn create_top_navbar() -> Box {
    let navbar = Box::new(Orientation::Horizontal, 10);
    navbar.set_margin_top(10);
    navbar.set_margin_start(10);
    navbar.set_margin_end(10);
    navbar.set_margin_bottom(10);
    let start_button = Button::with_label("Start Network");
    let stop_button = Button::with_label("Stop Network");
    let restart_button = Button::with_label("Restart Network");
    let status_label = Label::new(Some("Network Status:"));
    let status_indicator = Label::new(Some("●"));

    let network_selector = ComboBoxText::new();
    network_selector.append(Some("mainnet"), "Mainnet");
    network_selector.append(Some("testnet"), "Testnet");
    network_selector.append(Some("devnet"), "Devnet");
    network_selector.set_active_id(Some("testnet"));
    navbar.append(&start_button);
    navbar.append(&stop_button);
    navbar.append(&restart_button);
    navbar.append(&status_label);
    navbar.append(&status_indicator);
    navbar.append(&network_selector);

    navbar
}
