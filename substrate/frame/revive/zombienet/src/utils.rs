// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

//! Zombienet integration tests for pallet-revive.
//!
//! This crate contains integration tests that use Zombienet to test
//! pallet-revive functionality in a realistic multi-node environment.
use crate::TestEnvironment;
use anyhow::anyhow;
use pallet_revive::evm::{
	Account, Block as EvmBlock, BlockNumberOrTag, GenericTransaction, ReceiptInfo, TransactionInfo,
};
use pallet_revive_eth_rpc::{
	example::TransactionBuilder,
	subxt_client::{self},
	EthRpcClient,
};
use sp_core::{H256, U256};
use subxt::{
	self,
	config::polkadot::PolkadotExtrinsicParamsBuilder,
	dynamic::Value,
	ext::subxt_rpcs::rpc_params,
	tx::{DynamicPayload, TxProgress, TxStatus},
	OnlineClient, PolkadotConfig,
};
use subxt_signer::sr25519::Keypair;

const ROOT_FROM_NO_DATA: &str = "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421";

pub fn print_receipt_info(receipt: &ReceiptInfo) {
	println!("Receipt:");
	println!("- Block number:        {}", receipt.block_number);
	println!("- Block hash:          {}", receipt.block_hash);
	println!("- Gas used:            {}", receipt.gas_used);
	println!("- From:                {}", receipt.from);
	println!("- To:                  {:?}", receipt.to);
	println!("- Contract address:    {:?}", receipt.contract_address);
	println!("- Cumulative gas used: {}", receipt.cumulative_gas_used);
	println!("- Success:             {:?}", receipt.status);
}

pub async fn assert_block(
	test_env: &TestEnvironment,
	block_number_or_tag: BlockNumberOrTag,
	should_be_empty: bool,
) {
	let TestEnvironment { eth_rpc_client, collator_rpc_client, collator_client, .. } = test_env;

	println!("Asserting block {block_number_or_tag:?} should_be_empty: {should_be_empty}");
	let eth_rpc_block = eth_rpc_client
		.get_block_by_number(block_number_or_tag.clone(), false)
		.await
		.unwrap_or_else(|err| panic!("Failed to fetch block {block_number_or_tag:?}: {err:?}"))
		.expect(&format!("Expected block {block_number_or_tag:?} not found"));

	println!("eth block number: {:?} hash: {:?}", eth_rpc_block.number, eth_rpc_block.hash);

	if should_be_empty {
		// Blocks with no transactions and no state should have the same roots
		assert_eq!(hex::encode(&eth_rpc_block.transactions_root), ROOT_FROM_NO_DATA);
		assert_eq!(hex::encode(&eth_rpc_block.receipts_root), ROOT_FROM_NO_DATA);
		assert_eq!(hex::encode(&eth_rpc_block.state_root), ROOT_FROM_NO_DATA);
	}

	let substrate_block_hash: H256 = collator_rpc_client
		.request("chain_getBlockHash", rpc_params![eth_rpc_block.number])
		.await
		.unwrap_or_else(|err| {
			panic!("Failed to get block hash for block {:?}: {err:?}", eth_rpc_block.number)
		});

	println!("substrate block number: {:?} hash: {substrate_block_hash:?}", eth_rpc_block.number);
	let storage = collator_client.storage().at(substrate_block_hash);

	let query = subxt_client::storage().revive().ethereum_block();
	let evm_block: EvmBlock = storage
		.fetch(&query)
		.await
		.unwrap_or_else(|err| panic!("Failed to fetch EvmBlock from storage: {err:?}"))
		.expect("EvmBlock not found in storage")
		.0;
	assert_eq!(eth_rpc_block, evm_block);

	let number_u256 = subxt::utils::Static(eth_rpc_block.number);
	let query = subxt_client::storage().revive().block_hash(number_u256);

	let block_hash_from_storage: H256 = storage
		.fetch(&query)
		.await
		.unwrap_or_else(|err| panic!("Failed to fetch block hash from storage: {err:?}"))
		.expect(&format!("Block number {:?} hash not found in storage", eth_rpc_block.number));
	assert_eq!(eth_rpc_block.hash, block_hash_from_storage);
}

pub async fn assert_transactions(
	test_env: &TestEnvironment,
	signer: Account,
	transactions: Vec<(H256, GenericTransaction, ReceiptInfo)>,
) {
	let TestEnvironment { eth_rpc_client, .. } = test_env;

	for (tx_hash, tx, receipt) in transactions.into_iter() {
		let block_number = receipt.block_number;
		let block_hash = receipt.block_hash;
		let tx_unsigned = tx
			.try_into_unsigned()
			.unwrap_or_else(|err| panic!("Failed to convert transaction: {err:?}"));
		let tx_signed = signer.sign_transaction(tx_unsigned);
		let expected_tx_info = TransactionInfo::new(&receipt, tx_signed);

		let tx_by_hash = eth_rpc_client
			.get_transaction_by_hash(tx_hash)
			.await
			.unwrap_or_else(|err| panic!("Failed to fetch tx by hash {tx_hash:?}: {err:?}"))
			.expect(&format!("Expected transaction {tx_hash:?} not found"));
		let tx_by_block_number_and_index = eth_rpc_client
			.get_transaction_by_block_number_and_index(
				BlockNumberOrTag::U256(block_number.into()),
				receipt.transaction_index,
			)
			.await
			.unwrap_or_else(|err| {
				panic!(
					"Failed to fetch tx by block number {block_number:?} and index {:?} {err:?}",
					receipt.transaction_index
				)
			})
			.expect(&format!(
				"Expected transaction at block number {block_number:?} and index {:?} not found",
				receipt.transaction_index
			));
		let tx_by_block_hash_and_index = eth_rpc_client
			.get_transaction_by_block_hash_and_index(block_hash, receipt.transaction_index)
			.await
			.unwrap_or_else(|err| {
				panic!(
					"Failed to fetch tx by block hash {block_hash:?} and index {:?} {err:?}",
					receipt.transaction_index
				)
			})
			.expect(&format!(
				"Expected transaction at block hash {block_hash:?} and index {:?} not found",
				receipt.transaction_index
			));

		assert_eq!(expected_tx_info, tx_by_hash);
		assert_eq!(expected_tx_info, tx_by_block_number_and_index);
		assert_eq!(expected_tx_info, tx_by_block_hash_and_index);
	}
}

pub async fn eth_rpc_submit_transactions<Client: EthRpcClient + Sync + Send>(
	transactions: Vec<TransactionBuilder<Client>>,
) -> Result<
	Vec<(H256, GenericTransaction, pallet_revive_eth_rpc::example::SubmittedTransaction<Client>)>,
	anyhow::Error,
> {
	println!("Submitting {} EVM transactions", transactions.len());

	let mut submitted_txs = Vec::new();

	for tx_builder in transactions {
		let tx = tx_builder.send().await?;
		let hash = tx.hash();
		let generic_tx = tx.generic_transaction();
		println!("Submitted EVM tx: {:?}", hash);
		submitted_txs.push((hash, generic_tx, tx));
	}

	Ok(submitted_txs)
}

pub async fn eth_rpc_wait_for_receipts<Client: EthRpcClient + Sync + Send>(
	submitted_txs: Vec<(
		H256,
		GenericTransaction,
		pallet_revive_eth_rpc::example::SubmittedTransaction<Client>,
	)>,
) -> Result<Vec<(H256, GenericTransaction, ReceiptInfo)>, anyhow::Error> {
	let wait_futures: Vec<_> = submitted_txs
		.into_iter()
		.map(|(hash, generic_tx, tx)| async move {
			let receipt = tx.wait_for_receipt().await?;
			println!("Received receipt for tx: {:?} block: {:?}", hash, receipt.block_number);
			Ok::<(H256, GenericTransaction, ReceiptInfo), anyhow::Error>((
				hash, generic_tx, receipt,
			))
		})
		.collect();

	let results = futures::future::join_all(wait_futures).await;
	let results: Result<Vec<_>, _> = results.into_iter().collect();
	results
}

pub async fn substrate_submit_extrinsics(
	client: &OnlineClient<PolkadotConfig>,
	calls: Vec<DynamicPayload>,
	signer: &Keypair,
) -> Result<Vec<TxProgress<PolkadotConfig, OnlineClient<PolkadotConfig>>>, anyhow::Error> {
	println!("Submitting {} substrate extrinsics", calls.len());
	let mut nonce = client
		.tx()
		.account_nonce(&signer.public_key().into())
		.await
		.map_err(|err| anyhow!("Failed to fetch account nonce: {err:?}"))?;

	let mut submitted_txs = Vec::new();

	for call in calls {
		let extensions = PolkadotExtrinsicParamsBuilder::new().nonce(nonce).immortal().build();
		let tx = client
			.tx()
			.create_signed(&call, signer, extensions)
			.await?
			.submit_and_watch()
			.await?;

		println!("Submitted substrate extrinsic with nonce: {}", nonce);
		submitted_txs.push(tx);
		nonce += 1;
	}

	Ok(submitted_txs)
}

pub async fn substrate_wait_for_finalization(
	submitted_txs: Vec<TxProgress<PolkadotConfig, OnlineClient<PolkadotConfig>>>,
) -> Vec<Result<(), anyhow::Error>> {
	let wait_futures: Vec<_> = submitted_txs
		.into_iter()
		.enumerate()
		.map(|(index, mut tx)| async move {
			while let Some(status) = tx.next().await {
				let status = status?;
				match &status {
					TxStatus::InBestBlock(tx_in_block) |
					TxStatus::InFinalizedBlock(tx_in_block) => {
						let _result = tx_in_block.wait_for_success().await?;
						let block_status =
							if status.as_finalized().is_some() { "Finalized" } else { "Best" };
						println!(
							"[{}] Substrate tx {} in block: {:#?}",
							block_status,
							index,
							tx_in_block.block_hash()
						);
						return Ok(());
					},
					TxStatus::Error { message } |
					TxStatus::Invalid { message } |
					TxStatus::Dropped { message } => {
						return Err(anyhow::format_err!(
							"Error submitting substrate tx {}: {message}",
							index
						));
					},
					_ => continue,
				}
			}
			Ok(())
		})
		.collect();

	futures::future::join_all(wait_futures).await
}

/// Prepares a vector of EVM transaction builders for parallel execution.
/// Each transaction will have a sequential nonce starting from the provided base nonce.
pub async fn prepare_evm_transfer_transactions<Client: EthRpcClient + Sync + Send>(
	eth_rpc_client: &std::sync::Arc<Client>,
	signer: Account,
	recipient: &pallet_revive::evm::Address,
	amount: U256,
	num_transactions: usize,
) -> Result<Vec<TransactionBuilder<Client>>, anyhow::Error> {
	println!("Creating {} parallel transfer transactions", num_transactions);
	let mut nonce = eth_rpc_client
		.get_transaction_count(signer.address(), pallet_revive::evm::BlockTag::Latest.into())
		.await?;

	let mut transactions = Vec::new();
	for i in 0..num_transactions {
		let tx_builder = TransactionBuilder::new(eth_rpc_client)
			.signer(signer.clone())
			.nonce(nonce)
			.value(amount)
			.to(*recipient);

		transactions.push(tx_builder);
		println!("Prepared EVM transaction {}/{num_transactions} with nonce: {nonce:?}", i + 1);
		nonce = nonce.saturating_add(U256::one());
	}

	Ok(transactions)
}

/// Prepares a vector of substrate remark transactions for parallel execution.
pub fn prepare_substrate_remark_transactions(
	num_transactions: usize,
	remark_message: Option<&str>,
) -> Vec<DynamicPayload> {
	println!("Creating {} substrate remark transactions", num_transactions);
	let message = remark_message.unwrap_or("Hello there");
	let mut substrate_calls = Vec::new();

	for i in 0..num_transactions {
		let call = subxt::dynamic::tx("System", "remark", vec![Value::from_bytes(message)]);
		substrate_calls.push(call);
		println!("Prepared substrate transaction {}/{num_transactions}", i + 1);
	}

	substrate_calls
}
