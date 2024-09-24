use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SolanaNetworkManager {
    processes: Arc<Mutex<Vec<Option<Child>>>>, // Mutex to manage processes across threads
}

impl SolanaNetworkManager {
    // Create a new manager
    pub fn new() -> Self {
        SolanaNetworkManager {
            processes: Arc::new(Mutex::new(vec![None])),
        }
    }

    // Start the Solana Test Network
    pub fn start(&self) {
        let process_arc = Arc::clone(&self.processes);
        let ledger_path = ".ledger"; // Single ledger for simplicity
        let rpc_port = "8899";

        std::thread::spawn(move || {
            let cmd = Command::new("solana-test-validator")
                .args(&[
                    "--ledger", ledger_path,
                    "--rpc-port", rpc_port,
                    "--no-snapshot",
                    "--reset",
                    "--max-transaction-rate", "1000000", // Set airdrop rate limit to 1000000 SOL
                ])
                .stdout(Stdio::piped()) // Capture command output
                .spawn()
                .expect("Failed to start Solana Test Network");

            // Store the process handle
            let mut processes_lock = process_arc.lock().unwrap();
            processes_lock[0] = Some(cmd);
        });
    }

    // Stop the Solana Test Network
    pub fn stop(&self) {
        let cmd = Command::new("pkill")
            .arg("solana-test-validator")
            .stdout(Stdio::piped()) // Capture command output
            .spawn()
            .expect("Failed to stop Solana Test Network");
    }

    // Restart the Solana Test Network
    pub fn restart(&self) {
        self.stop(); // Stop the current network
        self.start(); // Start a new one
    }
    
}


pub fn get_wallet_balance(public_key:String) -> String {
    let cmd =  Command::new("solana").args(&["balance", public_key.as_str(),"--url","http://127.0.0.1:9090"]).output();
    let balance = match cmd {
        Ok(output)  => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if output.status.success() {
                stdout.to_string()
            }else {

                String::from("0 SOL")
            }

        },
        Err(_) => {
            String::from("0 SOL")
        }
    };
    println!("Balance : {:?}",balance);
    balance
}