use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Stack, StackSwitcher};

use super::{account_manager_ui, transaction_manager_ui,block_explorer_ui,terminal_ui};

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
    let block_explorer_page = block_explorer_ui::create_block_explorer_page();
    let account_manager_page = account_manager_ui::create_account_manager_page();
    let transaction_manager_page = transaction_manager_ui::create_transaction_manager_page();
    let terminal_page = terminal_ui::create_terminal_page();
    stack.add_titled(
        &block_explorer_page,
        Some("block_explorer"),
        "Block Explorer ",
    );
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
    stack.add_titled(
        &terminal_page,
        Some("terminal_manager"),
        "Terminal Manager",
    );
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
