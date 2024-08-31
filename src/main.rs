use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};
// use utils::utils
fn main() {
    let app = Application::new(
        Some("com.example.solana-ganache"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("PURPPLE");
        window.set_default_size(800, 600);

        let button = Button::with_label("Create Account");
        window.set_child(Some(&button));

        window.show();
    });

    app.run();
}
