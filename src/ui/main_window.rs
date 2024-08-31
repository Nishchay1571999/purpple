// src/ui/main_window.rs

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Stack, StackSwitcher};

use super::{account_manager_ui, transaction_manager_ui};

pub fn create_main_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("PURPPLE"));
    window.set_default_size(800, 600);

    let vbox = Box::new(Orientation::Vertical, 0);
    let stack = Stack::new();
    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));
    vbox.append(&stack_switcher);
    vbox.append(&stack);

    // Create and add pages to the stack
    let account_manager_page = account_manager_ui::create_account_manager_page();
    let transaction_manager_page = transaction_manager_ui::create_transaction_manager_page();

    stack.add_titled(
        &account_manager_page,
        Some("account_manager"),
        "Account Manager",
    );
    stack.add_titled(
        &transaction_manager_page,
        Some("transaction_manager"),
        "Transaction Manager",
    );
    // src/ui/main_window.rs

    stack.connect_visible_child_name_notify(|stack| {
        if let Some(name) = stack.visible_child_name() {
            match name.as_str() {
                "account_manager" => {
                    println!("Account Manager page is now visible");
                    // Refresh account data
                }
                "transaction_manager" => {
                    println!("Transaction Manager page is now visible");
                    // Refresh transaction data
                }
                _ => {}
            }
        }
    });

    window.set_child(Some(&vbox));
    window
}
