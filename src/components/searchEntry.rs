use gtk4::prelude::*;
use gtk4::SearchEntry;



pub fn create_new_search_entry(placeholder:&str)->SearchEntry {
    let search_bar = SearchEntry::new();
    search_bar.set_placeholder_text(Some(placeholder));
    search_bar.set_margin_top(10);
    search_bar.set_margin_start(10);
    search_bar.set_margin_end(10);
    search_bar.set_margin_bottom(10);

    return search_bar;
}