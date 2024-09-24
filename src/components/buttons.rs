use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Frame, Label, Orientation};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::process::{Command, Stdio};
use std::rc::Rc;

use crate::utils::logger::get_wallet_balance;

pub struct Keypair {
    private_key: String,
    public_key: String,
    balance: String, // Use Rc<RefCell<i32>> for mutable shared ownership
}

impl Keypair {
    pub fn new(public_key: String) -> Keypair {
        let balance = get_wallet_balance(public_key.clone());
        let private_key = String::from("");             
        Keypair {
            public_key,
            private_key,
            balance, // Initialize with balance 0
        }
    }

    pub fn get_widget(self) -> Box {
        let widget = Box::new(Orientation::Horizontal, 10);

        // Create a label displaying the public key
        let label = Label::new(Some(self.public_key.as_str()));

        // Set padding around the label (using margin)
        label.set_margin_start(10);
        label.set_margin_end(10);
        label.set_margin_top(5);
        label.set_margin_bottom(5);

        // Align the label to the start of the box (left)
        label.set_halign(Align::Start);

        // Create a button
        let button = Button::new();
        button.set_label("Key");
        button.set_margin_start(10);
        button.set_margin_end(10);
        button.set_margin_top(5);
        button.set_margin_bottom(5);
        button.set_width_request(60);
        button.set_halign(Align::End);
        button.set_hexpand(false);

        // Create an "Air Drop" button
        let air_drop_button = Button::new();
        air_drop_button.set_label("Air Drop");
        air_drop_button.set_margin_start(10);
        air_drop_button.set_margin_end(10);
        air_drop_button.set_margin_top(5);
        air_drop_button.set_margin_bottom(5);
        air_drop_button.set_width_request(60);
        air_drop_button.set_halign(Align::End);
        air_drop_button.set_hexpand(false);

        // Create a frame to display the balance
        let balance_frame = Frame::new(Some(self.balance.as_str()));
        let pub_key_clone = self.public_key;
        // Add widgets to the box
        widget.append(&label);
        widget.append(&button);
        widget.append(&air_drop_button);
        widget.append(&balance_frame);
        let balance_frame_clone = balance_frame.clone();

        // Connect the "clicked" signal for the airdrop button
        air_drop_button.connect_clicked(move |_| {
            let addr = pub_key_clone.clone();
            // Simulate airdrop (e.g., increment balance by 100)
            let cmd = Command::new("solana")
                .args(&["airdrop", "1", addr.as_str(), "--url", "http://127.0.0.1:9090"])
                .output();
            println!("Address {:?}", addr.as_str());
 
            // Check if the command was successful
            match cmd {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if output.status.success() {
                        println!("Airdrop successful: {}", stdout);
                        // Mutate the balance inside the RefCell
                        let balance_string = get_wallet_balance(addr);
                        // Update the balance display
                        balance_frame_clone.set_label(Some(balance_string.as_str()));
                    } else {
                        eprintln!("Airdrop failed: {}", stderr);
                        // If the command failed, print the error message from stderr
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Airdrop failed with error: {}", stderr);
                    }
                }
                Err(e) => {
                    // If the command itself failed to run, print the error
                    eprintln!("Failed to execute airdrop command: {}", e);
                }
            }
        });

        // Return the widget
        widget
    }
}
