use solana_rpc_client::rpc_client::RpcClient;

use solana_rpc_client_api::config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use std::thread;

use std::time::Duration;

fn main() {
    let rpc_client = RpcClient::new(
        "https://mainnet.helius-rpc.com/?api-key=ebda92cb-8ed0-420f-84bf-3e8b9c188973".to_string(),
    );

    let start_scan_block_height = rpc_client
        .get_block_height_with_commitment(CommitmentConfig::finalized())
        .unwrap();

    println!("{}", start_scan_block_height);
    let mut a_scan_block_height = start_scan_block_height + 1;
    let mut b_scan_block_height = start_scan_block_height + 2;
    let mut c_scan_block_height = start_scan_block_height + 3;

    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Base58),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(true),
        commitment: None,
        max_supported_transaction_version: Some(0),
    };

    let thread1 = thread::spawn(move || {
        let rpc_client = RpcClient::new(
            "https://mainnet.helius-rpc.com/?api-key=ebda92cb-8ed0-420f-84bf-3e8b9c188973"
                .to_string(),
        );

        loop {
            println!("processing block num: {}", a_scan_block_height);

            let block = rpc_client.get_block_with_config(a_scan_block_height, config.clone());
            if block.is_err() {
                println!("{:?}", block.err());
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            a_scan_block_height += 3;
        }
    });

    let thread2 = thread::spawn(move || {
        let rpc_client = RpcClient::new(
            "https://mainnet.helius-rpc.com/?api-key=ebda92cb-8ed0-420f-84bf-3e8b9c188973"
                .to_string(),
        );

        loop {
            println!("processing block num: {}", b_scan_block_height);

            let block = rpc_client.get_block_with_config(b_scan_block_height, config.clone());
            if block.is_err() {
                println!("{:?}", block.err());
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            b_scan_block_height += 3;
        }
    });

    let thread3 = thread::spawn(move || {
        let rpc_client = RpcClient::new(
            "https://mainnet.helius-rpc.com/?api-key=ebda92cb-8ed0-420f-84bf-3e8b9c188973"
                .to_string(),
        );

        loop {
            println!("processing block num: {}", c_scan_block_height);

            let block = rpc_client.get_block_with_config(c_scan_block_height, config);
            if block.is_err() {
                println!("{:?}", block.err());
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            c_scan_block_height += 3;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}
