use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Stack, StackSwitcher};

use super::{account_manager_ui, transaction_manager_ui,block_explorer_ui,terminal_ui};

pub fn create_stack_navigation(window:&ApplicationWindow,vbox: &Box)-> Stack {
    window.set_title(Some("PURPPLE"));
    window.set_default_size(800, 600);

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
        &account_manager_page,
        Some("account_manager"),
        "Account Manager",
    );
    stack.add_titled(
        &block_explorer_page,
        Some("block_explorer"),
        "Block Explorer ",
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
    return stack;
}