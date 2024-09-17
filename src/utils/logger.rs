use std::process::Command;

pub fn is_solana_installed() -> bool {
    Command::new("solana").arg("--version").output().is_ok()
}

pub fn start_test_solana_network() -> bool {
    Command::new("solana-test-validator")
        .args(&["--rpc-port", "9090"]) 
        .output()
        .is_ok()
}
