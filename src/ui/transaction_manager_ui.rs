use gtk4::prelude::*;
use gtk4::{
    Box, Button, CellRendererText, Entry, Label, ListStore, ListView, Orientation, ScrolledWindow,
    SearchEntry, TreeView, TreeViewColumn,
};

pub fn create_transaction_manager_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 10);

    // Create the search bar
    let search_bar = SearchEntry::new();
    search_bar.set_placeholder_text(Some("Search transactions..."));
    search_bar.set_margin_top(10);
    search_bar.set_margin_start(10);
    search_bar.set_margin_end(10);
    search_bar.set_margin_bottom(10);
    vbox.append(&search_bar);

    // Create the transaction history list
    let transaction_list_view = TreeView::new();
    let transaction_list_store = ListStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]); // Columns: ID, Date, Amount, Status

    let id_column = TreeViewColumn::new();
    id_column.set_title("Transaction ID");
    let id_renderer = CellRendererText::new();
    id_column.pack_start(&id_renderer, true);
    id_column.add_attribute(&id_renderer, "text", 0);
    transaction_list_view.append_column(&id_column);

    let date_column = TreeViewColumn::new();
    date_column.set_title("Date");
    let date_renderer = CellRendererText::new();
    date_column.pack_start(&date_renderer, true);
    date_column.add_attribute(&date_renderer, "text", 1);
    transaction_list_view.append_column(&date_column);

    let amount_column = TreeViewColumn::new();
    amount_column.set_title("Amount");
    let amount_renderer = CellRendererText::new();
    amount_column.pack_start(&amount_renderer, true);
    amount_column.add_attribute(&amount_renderer, "text", 2);
    transaction_list_view.append_column(&amount_column);

    let status_column = TreeViewColumn::new();
    status_column.set_title("Status");
    let status_renderer = CellRendererText::new();
    status_column.pack_start(&status_renderer, true);
    status_column.add_attribute(&status_renderer, "text", 3);
    transaction_list_view.append_column(&status_column);

    transaction_list_view.set_model(Some(&transaction_list_store));

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&transaction_list_view));
    scrolled_window.set_margin_top(10);
    scrolled_window.set_margin_start(10);
    scrolled_window.set_margin_end(10);
    scrolled_window.set_margin_bottom(10);
    vbox.append(&scrolled_window);

    // Create the transaction details panel
    let details_box = Box::new(Orientation::Vertical, 10);
    let id_label = Label::new(Some("Transaction ID:"));
    let sender_label = Label::new(Some("Sender:"));
    let receiver_label = Label::new(Some("Receiver:"));
    let amount_label = Label::new(Some("Amount:"));
    let date_label = Label::new(Some("Date:"));
    let status_label = Label::new(Some("Status:"));

    details_box.append(&id_label);
    details_box.append(&sender_label);
    details_box.append(&receiver_label);
    details_box.append(&amount_label);
    details_box.append(&date_label);
    details_box.append(&status_label);
    details_box.set_margin_top(10);
    details_box.set_margin_start(10);
    details_box.set_margin_end(10);
    details_box.set_margin_bottom(10);
    vbox.append(&details_box);

    // Create initiate new transaction form
    let initiate_box = Box::new(Orientation::Vertical, 10);
    let sender_entry = Entry::new();
    sender_entry.set_placeholder_text(Some("Sender Account"));
    let receiver_entry = Entry::new();
    receiver_entry.set_placeholder_text(Some("Receiver Account"));
    let amount_entry = Entry::new();
    amount_entry.set_placeholder_text(Some("Amount"));
    let note_entry = Entry::new();
    note_entry.set_placeholder_text(Some("Note"));

    let submit_button = Button::with_label("Submit Transaction");

    initiate_box.append(&sender_entry);
    initiate_box.append(&receiver_entry);
    initiate_box.append(&amount_entry);
    initiate_box.append(&note_entry);
    initiate_box.append(&submit_button);
    initiate_box.set_margin_top(10);
    initiate_box.set_margin_start(10);
    initiate_box.set_margin_end(10);
    initiate_box.set_margin_bottom(10);
    vbox.append(&initiate_box);

    // Create transaction status monitor
    let status_monitor_box = Box::new(Orientation::Vertical, 10);
    let monitor_label = Label::new(Some("Ongoing Transactions:"));

    status_monitor_box.append(&monitor_label);
    status_monitor_box.set_margin_top(10);
    status_monitor_box.set_margin_start(10);
    status_monitor_box.set_margin_end(10);
    status_monitor_box.set_margin_bottom(10);
    vbox.append(&status_monitor_box);

    // Add network status indicator (optional)
    let status_label = Label::new(Some("Network Status:"));
    let status_indicator = Label::new(Some("●"));
    status_indicator.set_css_classes(&["status-indicator"]); // Style using CSS
    let network_status_box = Box::new(Orientation::Horizontal, 10);
    network_status_box.append(&status_label);
    network_status_box.append(&status_indicator);
    network_status_box.set_margin_top(10);
    network_status_box.set_margin_start(10);
    network_status_box.set_margin_end(10);
    network_status_box.set_margin_bottom(10);
    vbox.append(&network_status_box);

    vbox
}
