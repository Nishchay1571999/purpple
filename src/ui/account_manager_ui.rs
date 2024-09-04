use gtk4::prelude::*;
use gtk4::{
    Box, Button, CellRendererText, ComboBoxText, Dialog, Entry, Label, ListStore, Orientation,
    ResponseType, ScrolledWindow, SearchEntry, TextView, TreeView, TreeViewColumn,
};

pub fn create_account_manager_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 10);

    let search_bar = SearchEntry::new();
    search_bar.set_placeholder_text(Some("Search accounts..."));
    search_bar.set_margin_top(10);
    search_bar.set_margin_start(10);
    search_bar.set_margin_end(10);
    search_bar.set_margin_bottom(10);
    vbox.append(&search_bar);

    let account_list_view = TreeView::new();
    let account_list_store = ListStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    let name_column = TreeViewColumn::new();
    name_column.set_title("Account Name");
    let name_renderer = CellRendererText::new();
    name_column.pack_start(&name_renderer, true);
    name_column.add_attribute(&name_renderer, "text", 0);
    account_list_view.append_column(&name_column);

    let address_column = TreeViewColumn::new();
    address_column.set_title("Address");
    let address_renderer = CellRendererText::new();
    address_column.pack_start(&address_renderer, true);
    address_column.add_attribute(&address_renderer, "text", 1);
    account_list_view.append_column(&address_column);

    let balance_column = TreeViewColumn::new();
    balance_column.set_title("Balance");
    let balance_renderer = CellRendererText::new();
    balance_column.pack_start(&balance_renderer, true);
    balance_column.add_attribute(&balance_renderer, "text", 2);
    account_list_view.append_column(&balance_column);

    account_list_view.set_model(Some(&account_list_store));

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&account_list_view));
    scrolled_window.set_margin_top(10);
    scrolled_window.set_margin_start(10);
    scrolled_window.set_margin_end(10);
    scrolled_window.set_margin_bottom(10);
    vbox.append(&scrolled_window);

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
    vbox.append(&details_box);

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
    vbox.append(&action_box);

    let status_label = Label::new(Some("Network Status:"));
    let status_indicator = Label::new(Some("●"));
    status_indicator.set_css_classes(&["status-indicator"]);
    let status_box = Box::new(Orientation::Horizontal, 10);
    status_box.append(&status_label);
    status_box.append(&status_indicator);
    status_box.set_margin_top(10);
    status_box.set_margin_start(10);
    status_box.set_margin_end(10);
    status_box.set_margin_bottom(10);
    vbox.append(&status_box);

    vbox
}
