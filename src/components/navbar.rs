use glib;
use gtk4::prelude::*;
use gtk4::{Box, Button, ComboBoxText, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;
use std::process::{Command, Child};
use std::sync::{Arc, Mutex};

pub struct NetworkState {
    validator_process: Option<Child>,
    is_running: bool,
}

pub fn create_block_explorer_top_navbar() -> Box {
    let navbar = Box::new(Orientation::Horizontal, 10);

    let start_button = Button::with_label("Start Network");
    let stop_button = Button::with_label("Stop Network");
    let restart_button = Button::with_label("Restart Network");
    let status_label = Label::new(Some("Network Status:"));
    let status_indicator = Rc::new(RefCell::new(Label::new(Some("●"))));
    let network_selector = ComboBoxText::new();
    network_selector.append(Some("mainnet"), "Mainnet");
    network_selector.append(Some("testnet"), "Testnet");
    network_selector.append(Some("devnet"), "Devnet");
    network_selector.set_active_id(Some("testnet"));

    // Create a shared network state
    let network_state = Arc::new(Mutex::new(NetworkState {
        validator_process: None,
        is_running: false,
    }));

    // Add callbacks to buttons
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

    // Append all widgets to the navbar
    navbar.append(&start_button);
    navbar.append(&stop_button);
    navbar.append(&restart_button);
    navbar.append(&status_label);
    navbar.append(&network_selector);

    navbar
}

fn start_solana_network(state: &Arc<Mutex<NetworkState>>, status_indicator: &Rc<RefCell<Label>>) {
    let mut state = state.lock().unwrap();
    if !state.is_running {
        match Command::new("solana-test-validator")
            .args(&["--rpc-port", "9090"])
            .spawn() {
                Ok(child) => {
                    state.validator_process = Some(child);
                    state.is_running = true;
                    status_indicator.borrow_mut().set_text("● Running");
                    status_indicator.borrow_mut().set_css_classes(&["running"]);
                    println!("Solana test validator started");
                },
                Err(e) => {
                    println!("Failed to start Solana test validator: {}", e);
                    status_indicator.borrow_mut().set_text("● Error");
                    status_indicator.borrow_mut().set_css_classes(&["error"]);
                }
            }
    }
}

fn stop_solana_network(state: &Arc<Mutex<NetworkState>>, status_indicator: &Rc<RefCell<Label>>) {
    let mut state = state.lock().unwrap();
    if state.is_running {
        if let Some(mut child) = state.validator_process.take() {
            match child.kill() {
                Ok(_) => {
                    state.is_running = false;
                    status_indicator.borrow_mut().set_text("● Stopped");
                    status_indicator.borrow_mut().set_css_classes(&["stopped"]);
                    println!("Solana test validator stopped");
                },
                Err(e) => {
                    println!("Failed to stop Solana test validator: {}", e);
                    status_indicator.borrow_mut().set_text("● Error");
                    status_indicator.borrow_mut().set_css_classes(&["error"]);
                }
            }
        }
    }
}

fn restart_solana_network(state: &Arc<Mutex<NetworkState>>, status_indicator: &Rc<RefCell<Label>>) {
    stop_solana_network(state, status_indicator);
    start_solana_network(state, status_indicator);
}