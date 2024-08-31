mod blockchain;
mod ui;
mod utils;

use blockchain::solana_client::SolanaClient;
use gtk4::prelude::*;
use gtk4::Application;

fn main() {
    let app = Application::new(Some("com.example.solana-ganache"), Default::default());

    app.connect_activate(|app| {
        let window = ui::main_window::create_main_window(app);
        window.show();
    });

    let solana_client = SolanaClient::new("http://localhost:8899");
    solana_client.get_latest_block();

    app.run();
}
