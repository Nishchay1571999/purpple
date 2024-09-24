use std::rc::Rc;

use crate::components::buttons::Keypair;
use crate::components::navbar::BlockExplorerNavbar;
use crate::components::search_entry;
use crate::utils::network::SolanaWalletManager;
use gtk4::{prelude::*, Label};
use gtk4::{Box, Button, Orientation};

pub struct AccountManager {
    container: Box,
}

impl AccountManager {
    pub fn new() -> Self {
        let vbox = Box::new(Orientation::Vertical,10    );

        let navbar = BlockExplorerNavbar::new();
        vbox.append(navbar.get_widget());
        let mut wallet_manager = SolanaWalletManager::new();
        wallet_manager.create_wallet();
        let mut addresses: Vec<String> = match wallet_manager.get_wallet_address() {
            Ok(addr) => addr,
            Err(_err) => Vec::new(),
        };
        if addresses.len() == 0 {
            addresses = wallet_manager.create_wallet();
        }
        let addres_box = Box::new(Orientation::Vertical,10);
        for addr in addresses.iter() {
            let label = Keypair::new(addr.to_string());
            addres_box.append(&label.get_widget());
        }
        vbox.append(&addres_box);
        let search_bar = search_entry::create_new_search_entry("Search some accounts ...");
        vbox.append(&search_bar);
        AccountManager { container: vbox }
    }

    pub fn get_widget(&self) -> &Box {
        &self.container
    }
}
