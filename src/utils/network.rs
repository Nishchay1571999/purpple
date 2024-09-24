use gtk4::Label;
use std::cell::RefCell;
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct NetworkState {
    pub validator_process: Option<Child>,
    pub is_running: bool,
}

impl NetworkState {
    pub fn new() -> Self {
        NetworkState {
            validator_process: None,
            is_running: false,
        }
    }
}

pub fn start_solana_network(
    state: &Arc<Mutex<NetworkState>>,
    status_indicator: &Rc<RefCell<Label>>,
) {
    let mut state = state.lock().unwrap();
    if !state.is_running {
        match Command::new("solana-test-validator")
            .args(&["--rpc-port", "9090"])
            .spawn()
        {
            Ok(child) => {
                state.validator_process = Some(child);
                state.is_running = true;
                status_indicator.borrow_mut().set_text("● Running");
                status_indicator.borrow_mut();
                println!("Solana test validator started");
            }
            Err(e) => {
                println!("Failed to start Solana test validator: {}", e);
                status_indicator.borrow_mut().set_text("● Error");
                status_indicator.borrow_mut();
            }
        }
    }
}

pub fn stop_solana_network(
    state: &Arc<Mutex<NetworkState>>,
    status_indicator: &Rc<RefCell<Label>>,
) {
    let mut state = state.lock().unwrap();
    if state.is_running {
        if let Some(mut child) = state.validator_process.take() {
            match child.kill() {
                Ok(_) => {
                    state.is_running = false;
                    status_indicator.borrow_mut().set_text("● Stopped");
                    status_indicator.borrow_mut();
                    println!("Solana test validator stopped");
                }
                Err(e) => {
                    println!("Failed to stop Solana test validator: {}", e);
                    status_indicator.borrow_mut().set_text("● Error");
                    status_indicator.borrow_mut();
                }
            }
        }
    }
}

pub fn restart_solana_network(
    state: &Arc<Mutex<NetworkState>>,
    status_indicator: &Rc<RefCell<Label>>,
) {
    stop_solana_network(state, status_indicator);
    start_solana_network(state, status_indicator);
}

pub struct SolanaWalletManager {
    address: Vec<String>, // RefCell to manage address
}

impl SolanaWalletManager {
    pub fn new() -> Self {
        SolanaWalletManager {
            address: Vec::new(),
        }
    }
    pub fn get_wallet_address(&self) -> Result<Vec<String>,bool>{
        if self.address.len() > 0 {
            return Ok(self.address.clone())
        }else {
            return Err(false)
        }
    }
    // Create a new Solana wallet and return its address
    pub fn create_wallet(&mut self) -> Vec<String> {
        // Define the path to the wallet file
        let mut addrs: Vec<String> = Vec::new();
        let wallet_path = "/tmp/";

        // Start the wallet creation process
        for digit in 10..20 {
            // let wallet = KeyPair
            
            let json_file = format!("{}{}-solana-wallet.json", wallet_path.clone(), digit.clone());
            let mut cmd = Command::new("solana-keygen")
                .args(&["new", "--outfile", json_file.as_str()])
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|e| e.to_string()).unwrap();
            // Get a mutable reference to the process's stdin
            let stdin = cmd.stdin.as_mut().ok_or("Failed to open stdin").unwrap();

            // Simulate interactive input
            writeln!(stdin, "yes").map_err(|e| e.to_string()).unwrap();
            // Wait for the process to complete
            let _ = cmd.wait().map_err(|e| e.to_string()).unwrap();
            
            // Get the address from the wallet file
            let output = Command::new("solana-keygen")
                .args(&["pubkey", &json_file])
                .output()
                .map_err(|e| e.to_string())
                .unwrap();

            // Check if the command was successful
            if !output.status.success() {
                println!("Error ");
            }

            // Convert the output to a String
            let address = String::from_utf8_lossy(&output.stdout).trim().to_string();
            addrs.push(address);
        }
        self.address = addrs.clone();
        return addrs;
    }

    // Retrieve the wallet address
}
