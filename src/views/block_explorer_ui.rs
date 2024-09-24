use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

pub struct BlockExplorer {
    pub container: Box,
}

impl BlockExplorer {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 10);

        let block_label = Label::new(Some("Blocks"));
        let block_content = Label::new(Some("Block data will be displayed here."));

        let tx_label = Label::new(Some("Transactions"));
        let tx_content = Label::new(Some("Transaction data will be displayed here."));

        container.append(&block_label);
        container.append(&block_content);
        container.append(&tx_label);
        container.append(&tx_content);

        BlockExplorer { container }
    }

    pub fn get_widget(&self) -> &Box {
        &self.container
    }
}
