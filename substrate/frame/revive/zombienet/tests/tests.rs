// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

use pallet_revive::evm::{Account, BlockNumberOrTag, BlockTag};
use pallet_revive_eth_rpc::{example::TransactionBuilder, EthRpcClient};
use pallet_revive_zombienet::{utils::*, TestEnvironment, BEST_BLOCK_METRIC};
use sp_core::U256;
use subxt_signer::sr25519::dev;

const COLLATOR_RPC_PORT: u16 = 9944;
const ETH_RPC_URL: &str = "http://localhost:8545";

// This tests makes sure that RPC collator is able to build blocks
#[tokio::test(flavor = "multi_thread")]
async fn test_dont_spawn_zombienet() {
	let _ = env_logger::try_init_from_env(
		env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
	);

	let test_env = TestEnvironment::without_zombienet(COLLATOR_RPC_PORT, ETH_RPC_URL)
		.await
		.unwrap_or_else(|err| panic!("Failed to create test environment: {err:?}"));

	// TODO: block zero is reconstructed from substrate
	// sanity_block_check(&test_env, BlockNumberOrTag::U256(0.into()), true).await;
	assert_block(&test_env, BlockNumberOrTag::U256(1.into()), true).await;
	assert_block(&test_env, BlockNumberOrTag::BlockTag(BlockTag::Earliest), true).await;
	assert_block(&test_env, BlockNumberOrTag::BlockTag(BlockTag::Finalized), true).await;

	test_single_transfer(&test_env).await;
	test_deployment(&test_env).await;
	test_parallel_transfers(&test_env, 5).await;
	test_mixed_evm_substrate_transactions(&test_env, 10, 10).await;
}

// This tests makes sure that RPC collator is able to build blocks
#[tokio::test(flavor = "multi_thread")]
async fn test_with_zombienet_spawning() {
	let _ = env_logger::try_init_from_env(
		env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
	);

	let test_env = TestEnvironment::with_zombienet(COLLATOR_RPC_PORT, ETH_RPC_URL)
		.await
		.unwrap_or_else(|err| panic!("Failed to create test environment: {err:?}"));
	let zombienet = test_env.zombienet.as_ref().unwrap();

	// TODO: block zero is reconstructed from substrate
	// sanity_block_check(&test_env, BlockNumberOrTag::U256(0.into()), true).await;
	// assert_block(&test_env, BlockNumberOrTag::U256(1.into()), true).await;
	// assert_block(&test_env, BlockNumberOrTag::BlockTag(BlockTag::Earliest), true).await;
	// assert_block(&test_env, BlockNumberOrTag::BlockTag(BlockTag::Finalized), true).await;

	// test_transfer(&test_env).await;

	// TODO remove after tests are implemented
	let alice = zombienet
		.network
		.get_node("alice-westend-validator")
		.unwrap_or_else(|err| panic!("Failed to get node: {err:?}"));
	assert!(alice
		.wait_metric_with_timeout(BEST_BLOCK_METRIC, |b| b >= 600.0, 3600u64)
		.await
		.is_ok());
}

async fn test_single_transfer(test_env: &TestEnvironment) {
	let TestEnvironment { eth_rpc_client, .. } = test_env;

	let alith = Account::default();
	let alith_address = alith.address();
	let ethan = Account::from(subxt_signer::eth::dev::ethan());
	let amount = 1_000_000_000_000_000_000_000u128.into();

	let alith_balance_before = eth_rpc_client
		.get_balance(alith_address, BlockTag::Latest.into())
		.await
		.unwrap_or_else(|err| panic!("Failed to get Alith's balance: {err:?}"));
	let ethan_balance_before = eth_rpc_client
		.get_balance(ethan.address(), BlockTag::Latest.into())
		.await
		.unwrap_or_else(|err| panic!("Failed to get Ethan's balance: {err:?}"));

	println!("\n\n=== Transferring  ===\n\n");

	let transactions = prepare_evm_transfer_transactions(
		eth_rpc_client,
		alith.clone(),
		&ethan.address(),
		amount,
		1,
	)
	.await
	.unwrap_or_else(|err| panic!("Failed to prepare EVM transactions: {err:?}"));

	// Submit all transactions
	let submitted_txs = eth_rpc_submit_transactions(transactions)
		.await
		.unwrap_or_else(|err| panic!("Failed to submit transactions: {err:?}"));

	// Wait for all receipts
	let (tx_hash, generic_tx, receipt) = eth_rpc_wait_for_receipts(submitted_txs)
		.await
		.unwrap_or_else(|err| panic!("Failed to wait for parallel transactions: {err:?}"))
		.pop()
		.expect("Expected vector of lenght 1");

	print_receipt_info(&receipt);

	let alith_balance_after = eth_rpc_client
		.get_balance(alith_address, BlockTag::Latest.into())
		.await
		.unwrap_or_else(|err| panic!("Failed to get Alith's balance: {err:?}"));
	let ethan_balance_after = eth_rpc_client
		.get_balance(ethan.address(), BlockTag::Latest.into())
		.await
		.unwrap_or_else(|err| panic!("Failed to get Ethan's balance: {err:?}"));
	println!("Balances before:");
	println!("  Alith: {alith_balance_before:?}");
	println!("  Ethan: {ethan_balance_before:?}");
	println!("Balances after:");
	println!(
		"  Alith: {alith_balance_after:?} gas:{:?}",
		alith_balance_before.saturating_sub(alith_balance_after).saturating_sub(amount)
	);
	println!("  Ethan: {ethan_balance_after:?}");

	// TODO:
	//  Should Alith's balance reduced by amount and gas used?
	//  Currently gas used is zero
	// assert_eq!(
	// 	alith_balance_after,
	// 	alith_balance_before.saturating_sub(amount).saturating_sub(gas_used)
	// );
	assert_eq!(ethan_balance_after, ethan_balance_before.saturating_add(amount));
	assert_block(test_env, BlockNumberOrTag::U256(receipt.block_number), false).await;
	assert_transactions(test_env, alith, vec![(tx_hash, generic_tx, receipt)]).await;
}

async fn test_deployment(test_env: &TestEnvironment) {
	let TestEnvironment { eth_rpc_client, .. } = test_env;

	let account = Account::default();

	let data = vec![];
	let (bytes, _) = pallet_revive_fixtures::compile_module("dummy")
		.unwrap_or_else(|err| panic!("Failed to compile dummy contract: {err:?}"));
	let input = bytes.into_iter().chain(data.clone()).collect::<Vec<u8>>();

	println!("Account:");
	println!("- address: {:?}", account.address());
	println!("- substrate: {}", account.substrate_account());

	println!("\n\n=== Deploying contract ===\n\n");

	let nonce = eth_rpc_client
		.get_transaction_count(account.address(), BlockTag::Latest.into())
		.await
		.unwrap_or_else(|err| panic!("Failed to get transactions count: {err:?}"));

	let tx = TransactionBuilder::new(&eth_rpc_client)
		.signer(account.clone())
		.value(5_000_000_000_000u128.into())
		.input(input)
		.send()
		.await
		.unwrap_or_else(|err| panic!("Failed to send transaction: {err:?}"));
	println!("Tx hash: {:?}", tx.hash());

	let receipt = tx
		.wait_for_receipt()
		.await
		.unwrap_or_else(|err| panic!("Failed while waiting for receipt: {err:?}"));
	print_receipt_info(&receipt);

	let contract_address = receipt.contract_address.unwrap();

	assert_eq!(
		contract_address,
		pallet_revive::create1(&account.address(), nonce.try_into().unwrap())
	);
	assert_block(test_env, BlockNumberOrTag::U256(receipt.block_number), false).await;
	assert_transactions(
		test_env,
		account.clone(),
		vec![(tx.hash(), tx.generic_transaction(), receipt)],
	)
	.await;

	println!("\n\n=== Calling contract ===\n\n");
	let tx = TransactionBuilder::new(&eth_rpc_client)
		.value(U256::from(1_000_000u32))
		.to(contract_address)
		.send()
		.await
		.unwrap_or_else(|err| panic!("Failed to send transaction: {err:?}"));
	println!("Tx hash: {:?}", tx.hash());
	let receipt = tx
		.wait_for_receipt()
		.await
		.unwrap_or_else(|err| panic!("Failed while waiting for receipt: {err:?}"));
	print_receipt_info(&receipt);

	assert_eq!(contract_address, receipt.to.unwrap());
	assert_block(test_env, BlockNumberOrTag::U256(receipt.block_number), false).await;
	assert_transactions(test_env, account, vec![(tx.hash(), tx.generic_transaction(), receipt)])
		.await;
}

async fn test_parallel_transfers(test_env: &TestEnvironment, num_transactions: usize) {
	println!("\n\n=== Testing Parallel Transfers ===\n\n");

	let TestEnvironment { eth_rpc_client, .. } = test_env;
	let alith = Account::default();
	let ethan = Account::from(subxt_signer::eth::dev::ethan());
	let amount = U256::from(1_000_000_000_000_000_000u128);

	let transactions = prepare_evm_transfer_transactions(
		eth_rpc_client,
		alith.clone(),
		&ethan.address(),
		amount,
		num_transactions,
	)
	.await
	.unwrap_or_else(|err| panic!("Failed to prepare EVM transactions: {err:?}"));

	// Submit all transactions
	let submitted_txs = eth_rpc_submit_transactions(transactions)
		.await
		.unwrap_or_else(|err| panic!("Failed to submit transactions: {err:?}"));

	// Wait for all receipts
	let results = eth_rpc_wait_for_receipts(submitted_txs)
		.await
		.unwrap_or_else(|err| panic!("Failed to wait for parallel transactions: {err:?}"));

	println!("Successfully completed {} parallel transactions", results.len());

	let mut blocks = vec![];
	let mut txs = vec![];

	// Verify all transactions were successful
	for (i, (hash, generic_tx, receipt)) in results.into_iter().enumerate() {
		println!("Transaction {}: hash={hash:?}, block={}", i + 1, receipt.block_number);
		assert_eq!(
			receipt.status.unwrap_or(U256::zero()),
			U256::one(),
			"Transaction should be successful"
		);
		let block = BlockNumberOrTag::U256(receipt.block_number);

		if !blocks.contains(&block) {
			blocks.push(block);
		}
		txs.push((hash, generic_tx, receipt));
	}

	for block in blocks {
		assert_block(test_env, block, false).await;
	}
	assert_transactions(test_env, alith, txs).await;
}

async fn test_mixed_evm_substrate_transactions(
	test_env: &TestEnvironment,
	num_evm_txs: usize,
	num_substrate_txs: usize,
) {
	println!("\n\n=== Testing Mixed EVM and Substrate Transactions ===\n\n");

	let TestEnvironment { eth_rpc_client, collator_client, .. } = test_env;
	let alith = Account::default();
	let ethan = Account::from(subxt_signer::eth::dev::ethan());
	let amount = U256::from(500_000_000_000_000_000u128);

	// Prepare EVM transactions
	let evm_transactions = prepare_evm_transfer_transactions(
		eth_rpc_client,
		alith.clone(),
		&ethan.address(),
		amount,
		num_evm_txs,
	)
	.await
	.unwrap_or_else(|err| panic!("Failed to prepare EVM transactions: {err:?}"));

	// Prepare substrate transactions (simple remarks)
	let alice_signer = dev::alice();
	let substrate_calls = prepare_substrate_remark_transactions(num_substrate_txs, None);

	println!(
		"Submitting {} EVM and {} substrate transactions synchronously, then waiting in parallel",
		num_evm_txs, num_substrate_txs
	);

	// Submit transactions
	let evm_submitted = eth_rpc_submit_transactions(evm_transactions)
		.await
		.unwrap_or_else(|err| panic!("Failed to submit EVM transactions: {err:?}"));
	let substrate_submitted =
		substrate_submit_extrinsics(collator_client, substrate_calls, &alice_signer)
			.await
			.unwrap_or_else(|err| panic!("Failed to submit substrate transactions: {err:?}"));

	// Wait for all transactions in parallel
	let (evm_results, substrate_results) = tokio::join!(
		eth_rpc_wait_for_receipts(evm_submitted),
		substrate_wait_for_finalization(substrate_submitted)
	);

	// Handle results
	let evm_results = evm_results
		.unwrap_or_else(|err| panic!("Failed to submit or wait for EVM transactions: {err:?}"));

	let substrate_success_count = substrate_results.iter().filter(|result| result.is_ok()).count();
	let substrate_failed_count = substrate_results.len() - substrate_success_count;

	println!(
		"Completed {} EVM and {} substrate transactions ({} substrate failed))",
		evm_results.len(),
		substrate_success_count,
		substrate_failed_count,
	);

	// Report any substrate transaction failures
	for (i, result) in substrate_results.iter().enumerate() {
		if let Err(err) = result {
			println!("Substrate transaction {} failed: {err:?}", i + 1);
		}
	}

	// Verify EVM transactions
	let mut blocks = vec![];
	let mut evm_txs = vec![];

	for (i, (hash, generic_tx, receipt)) in evm_results.into_iter().enumerate() {
		println!("EVM Transaction {}: hash={hash:?}, block={}", i + 1, receipt.block_number);
		assert_eq!(
			receipt.status.unwrap_or(U256::zero()),
			U256::one(),
			"EVM transaction should be successful"
		);
		let block = BlockNumberOrTag::U256(receipt.block_number);

		if !blocks.contains(&block) {
			blocks.push(block);
		}
		evm_txs.push((hash, generic_tx, receipt));
	}

	// Verify blocks contain the transactions
	for block in blocks {
		assert_block(test_env, block, false).await;
	}
	assert_transactions(test_env, alith, evm_txs).await;

	println!(
		"Successfully completed mixed transaction test with {} EVM and {} substrate transactions",
		num_evm_txs, substrate_success_count
	);
}
