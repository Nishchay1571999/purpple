use gtk4::prelude::*;
use gtk4::Entry;

pub fn create_new_search_entry(placeholder_text: &str) -> Entry {
    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some(placeholder_text));
    search_entry
}
