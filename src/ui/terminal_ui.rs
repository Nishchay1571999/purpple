use gtk4::prelude::*;
use gtk4::{
    Box, Button, CellRendererText, Entry, Label, ListStore, Orientation, ScrolledWindow,
    SearchEntry, TextView, TreeView, TreeViewColumn,
};

pub fn create_terminal_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 10);

    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search logs/events..."));
    search_entry.set_margin_top(10);
    search_entry.set_margin_start(10);
    search_entry.set_margin_end(10);
    search_entry.set_margin_bottom(10);
    vbox.append(&search_entry);

    let log_text_view = TextView::new();
    log_text_view.set_vexpand(true);
    let log_scrolled_window = ScrolledWindow::new();
    log_scrolled_window.set_child(Some(&log_text_view));
    log_scrolled_window.set_margin_top(10);
    log_scrolled_window.set_margin_start(10);
    log_scrolled_window.set_margin_end(10);
    log_scrolled_window.set_margin_bottom(10);
    vbox.append(&log_scrolled_window);

    let event_list_view = TreeView::new();
    let event_list_store = ListStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    let type_column = TreeViewColumn::new();
    type_column.set_title("Event Type");
    let type_renderer = CellRendererText::new();
    type_column.pack_start(&type_renderer, true);
    type_column.add_attribute(&type_renderer, "text", 0);
    event_list_view.append_column(&type_column);

    let timestamp_column = TreeViewColumn::new();
    timestamp_column.set_title("Timestamp");
    let timestamp_renderer = CellRendererText::new();
    timestamp_column.pack_start(&timestamp_renderer, true);
    timestamp_column.add_attribute(&timestamp_renderer, "text", 1);
    event_list_view.append_column(&timestamp_column);

    let details_column = TreeViewColumn::new();
    details_column.set_title("Details");
    let details_renderer = CellRendererText::new();
    details_column.pack_start(&details_renderer, true);
    details_column.add_attribute(&details_renderer, "text", 2);
    event_list_view.append_column(&details_column);

    event_list_view.set_model(Some(&event_list_store));

    let event_scrolled_window = ScrolledWindow::new();
    event_scrolled_window.set_child(Some(&event_list_view));
    event_scrolled_window.set_margin_top(10);
    event_scrolled_window.set_margin_start(10);
    event_scrolled_window.set_margin_end(10);
    event_scrolled_window.set_margin_bottom(10);
    vbox.append(&event_scrolled_window);

    let status_box = Box::new(Orientation::Horizontal, 10);
    let peer_status_label = Label::new(Some("Connected Peers:"));
    let node_health_label = Label::new(Some("Node Health:"));
    let status_indicator = Label::new(Some("●"));

    status_box.append(&peer_status_label);
    status_box.append(&node_health_label);
    status_box.append(&status_indicator);
    status_box.set_margin_top(10);
    status_box.set_margin_start(10);
    status_box.set_margin_end(10);
    status_box.set_margin_bottom(10);
    vbox.append(&status_box);

    let command_entry = Entry::new();
    command_entry.set_placeholder_text(Some("Enter command..."));
    let send_command_button = Button::with_label("Send Command");

    let command_box = Box::new(Orientation::Horizontal, 10);
    command_box.append(&command_entry);
    command_box.append(&send_command_button);
    command_box.set_margin_top(10);
    command_box.set_margin_start(10);
    command_box.set_margin_end(10);
    command_box.set_margin_bottom(10);
    vbox.append(&command_box);

    let refresh_button = Button::with_label("Refresh");
    refresh_button.set_margin_top(10);
    refresh_button.set_margin_start(10);
    refresh_button.set_margin_end(10);
    refresh_button.set_margin_bottom(10);
    vbox.append(&refresh_button);

    vbox
}
