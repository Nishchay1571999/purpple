use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

pub fn create_main_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("PURPPLE"));
    window.set_default_size(800, 600);
    let button = gtk4::Button::with_label("Manage Accounts");
    window.set_child(Some(&button));

    window
}
