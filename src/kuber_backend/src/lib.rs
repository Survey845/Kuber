use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Wallet {
    balance: u64,
}

type Address = String;

// Wallet Storage
thread_local! {
    static WALLETS: RefCell<HashMap<Address, Wallet>> = RefCell::new(HashMap::new());
}

// Initialize Wallet
#[init]
fn init() {
    WALLETS.with(|wallets| {
        wallets.borrow_mut().clear(); 
    });
}

// Get Balance
#[query(name="get_balance")]
fn get_balance(address: Address) -> u64 {
    WALLETS.with(|wallets| {
        wallets.borrow().get(&address).map_or(0, |wallet| wallet.balance)
    })
}

// Send Tokens
#[update(name="send_tokens")]
fn send_tokens(from: Address, to: Address, amount: u64) -> Result<(), String> {
    WALLETS.with(|wallets| {
        let mut wallets = wallets.borrow_mut();

        if let Some(sender_wallet) = wallets.get_mut(&from) {
            if sender_wallet.balance < amount {
                return Err("Insufficient balance".to_string());
            }
            sender_wallet.balance -= amount;

            // Add tokens to the receiver's wallet
            let receiver_wallet = wallets.entry(to.clone()).or_insert(Wallet { balance: 0 });
            receiver_wallet.balance += amount;

            Ok(())
        } else {
            Err("Sender wallet not found".to_string())
        }
    })
}

// Receive Tokens
#[update(name="receive_tokens")]
fn receive_tokens(address: Address, amount: u64) -> Result<(), String> {
    WALLETS.with(|wallets| {
        let mut wallets = wallets.borrow_mut();
        let wallet = wallets.entry(address).or_insert(Wallet { balance: 0 });
        wallet.balance += amount;
        Ok(())
    })
}