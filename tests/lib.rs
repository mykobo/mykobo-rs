mod anchor;
mod business;
mod identity;
mod ledger;
mod message_bus;
mod models;
mod sumsub;
mod wallets;

use std::fs;

fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
