use gtk4::prelude::*;
use gtk4::{Box, Button, ComboBoxText, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::utils::network::{restart_solana_network, start_solana_network, stop_solana_network, NetworkState};

pub struct BlockExplorerNavbar {
    pub container: Box,
}

impl BlockExplorerNavbar {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 10);
        let start_button = Button::with_label("Start Network");
        let stop_button = Button::with_label("Stop Network");
        let restart_button = Button::with_label("Restart Network");
        let status_label = Label::new(Some("Network Status:"));
        let status_indicator = Rc::new(RefCell::new(Label::new(Some("●"))));
        let network_selector = ComboBoxText::new();
        network_selector.append(Some("mainnet"), "Mainnet");
        network_selector.set_active_id(Some("testnet"));

        let network_state = Arc::new(Mutex::new(NetworkState::new()));

        // Button Callbacks
        let ns_clone = network_state.clone();
        let si_clone = status_indicator.clone();
        start_button.connect_clicked(move |_| {
            start_solana_network(&ns_clone, &si_clone);
        });

        let ns_clone = network_state.clone();
        let si_clone = status_indicator.clone();
        stop_button.connect_clicked(move |_| {
            stop_solana_network(&ns_clone, &si_clone);
        });

        let ns_clone = network_state.clone();
        let si_clone = status_indicator.clone();
        restart_button.connect_clicked(move |_| {
            restart_solana_network(&ns_clone, &si_clone);
        });

        // Add widgets to the container
        container.append(&start_button);
        container.append(&stop_button);
        container.append(&restart_button);
        container.append(&status_label);
        container.append(&network_selector);
        BlockExplorerNavbar {
            container,
        }
    }

    pub fn get_widget(&self) -> &Box {
        &self.container
    }
}
