use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box, Stack, StackSwitcher};

use super::account_manager_ui::AccountManager;
use super::block_explorer_ui::BlockExplorer;
use super::transaction_manager_ui::TransactionManager;

pub fn create_stack_navigation(window: &ApplicationWindow, vbox: &Box) -> Stack {
    window.set_title(Some("PURPPLE"));
    window.set_default_size(800, 600);

    let stack = Stack::new();
    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));
    vbox.append(&stack_switcher);
    vbox.append(&stack);
    let account_manger_page = AccountManager::new();
    let block_explorer_page = BlockExplorer::new();
    let transaction_manager_page = TransactionManager::new();
    stack.add_titled(
        account_manger_page.get_widget(),
        Some("account_manager"),
        "Account Manager",
    );
    stack.add_titled(
        block_explorer_page.get_widget(),
        Some("block_explorer"),
        "Block Explorer",
    );
    stack.add_titled(
        transaction_manager_page.get_widget(),
        Some("transaction_manager"),
        "Transaction Manager",
    );

    return stack;
}
