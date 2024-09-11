use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Stack, StackSwitcher};

use super::navigation;

pub fn create_main_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    let vbox = Box::new(Orientation::Vertical, 0);

    let stack = navigation::create_stack_navigation(&window,&vbox);
    stack.connect_visible_child_name_notify(|stack| {
        if let Some(name) = stack.visible_child_name() {
            match name.as_str() {
                "account_manager" => {
                    println!("Account Manager page is now visible");
                }
                "transaction_manager" => {
                    println!("Transaction Manager page is now visible");
                }
                "block_explorer" => {
                    println!("Block Explorer page is now visible");
                }
                "transaction_manager" =>{
                    println!("Terminal Manager")
                }
                _ => {}
            }
        }
    });

    window.set_child(Some(&vbox));
    window
}
