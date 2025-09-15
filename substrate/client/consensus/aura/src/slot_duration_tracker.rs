// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Module implementing the logic for verifying and importing AuRa blocks.

use std::{fmt::Debug, sync::Arc};

use codec::Codec;
use fork_tree::ForkTree;
use parking_lot::RwLock;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{HeaderBackend, HeaderMetadata};
use sp_consensus_aura::{AuraApi, SlotDuration};
use sp_core::Pair;
use sp_runtime::traits::{Block, Header, NumberFor};

use crate::AuthorityId;

/// AURA slot duration tracker. Updates authorities based on information from the runtime.
pub struct SlotDurationTracker<P, B: Block, C> {
	slot_durations: RwLock<ForkTree<B::Hash, NumberFor<B>, SlotDuration>>,
	client: Arc<C>,
	_phantom: std::marker::PhantomData<P>,
}

impl<P: Pair, B: Block, C> SlotDurationTracker<P, B, C>
where
	C: HeaderBackend<B> + HeaderMetadata<B, Error = sp_blockchain::Error> + ProvideRuntimeApi<B>,
	P::Public: Codec + Debug,
	C::Api: AuraApi<B, AuthorityId<P>>,
{
	/// Create a new `SlotDurationTracker`.
	pub fn new(client: Arc<C>) -> Result<Self, String> {
		let finalized_hash = client.info().finalized_hash;
		let mut slot_durations = ForkTree::new();
		for mut hash in
			client.leaf_hashes().map_err(|e| format!("Could not get leaf hashes: {e}"))?
		{
			// Import the entire chain back to the first imported ancestor, or to the last finalized
			// block if there is no imported ancestor. The chain must be imported in order, from
			// first block to last.
			let mut chain = Vec::new();
			loop {
				let header = client
					.header(hash)
					.map_err(|e| format!("Could not get header for {hash:?}: {e}"))?
					.ok_or_else(|| format!("Header for {hash:?} not found"))?;
				let number = *header.number();
				let is_descendent_of = sc_client_api::utils::is_descendent_of(&*client, None);
				let existing_node =
					slot_durations
						.find_node_where(&hash, &number, &is_descendent_of, &|_| true)
						.map_err(|e| {
							format!("Could not find authorities for block {hash:?} at number {number}: {e}")
						})?;
				if existing_node.is_some() {
					// We have already imported this part of the chain.
					break;
				}
				chain.push((number, hash));
				if hash == finalized_hash {
					break;
				}
				hash = *header.parent_hash();
			}
			let mut last_imported_slot_duration = None;
			for (number, hash) in chain.into_iter().rev() {
				let slot_duration = client.runtime_api().slot_duration(hash).map_err(|e| {
					format!("Could not get slot duration from runtime at {hash:?}: {e}")
				})?;
				if Some(&slot_duration) != last_imported_slot_duration.as_ref() {
					last_imported_slot_duration = Some(slot_duration.clone());
					let is_descendent_of = sc_client_api::utils::is_descendent_of(&*client, None);
					slot_durations.import(hash, number, slot_duration, &is_descendent_of).map_err(
						|e| {
							format!("Could not import authorities for block {hash:?} at number {number}: {e}")
						},
					)?;
				}
			}
		}
		Ok(Self {
			slot_durations: RwLock::new(slot_durations),
			client,
			_phantom: std::marker::PhantomData,
		})
	}
}

impl<P, B, C> SlotDurationTracker<P, B, C>
where
	P: Pair,
	B: Block,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = sp_blockchain::Error> + ProvideRuntimeApi<B>,
	P::Public: Codec + Debug,
	C::Api: AuraApi<B, AuthorityId<P>>,
{
	/// Fetch the slot duration from the tracker, if available. If not available, return an error.
	pub fn fetch(&self, header: &B::Header) -> Result<SlotDuration, String> {
		let hash = header.hash();
		let number = *header.number();
		let parent_hash = *header.parent_hash();
		let is_descendent_of =
			sc_client_api::utils::is_descendent_of(&*self.client, Some((hash, parent_hash)));
		let slot_durations = self.slot_durations.read();
		let node = slot_durations
			.find_node_where(&hash, &number, &is_descendent_of, &|_| true)
			.map_err(|e| {
				format!("Could not find authorities for block {hash:?} at number {number}: {e}")
			})?
			.ok_or_else(|| {
				format!("Authorities for block {hash:?} at number {number} not found in",)
			})?;
		Ok(node.data.clone())
	}

	/// Import the slot duration from the runtime for the given header.
	pub fn import(&self, header: &B::Header) -> Result<(), String> {
		let hash = header.hash();
		let number = *header.number();
		let slot_duration = self.client.runtime_api().slot_duration(hash).map_err(|e| {
			format!("Could not get slot duration from runtime at {}: {e}", header.hash())
		})?;
		self.prune_finalized()?;
		let is_descendent_of = sc_client_api::utils::is_descendent_of(&*self.client, None);
		let mut slot_durations = self.slot_durations.write();
		slot_durations
			.import(hash, number, slot_duration, &is_descendent_of)
			.map_err(|e| {
				format!("Could not import authorities for block {hash:?} at number {number}: {e}")
			})?;
		Ok(())
	}

	fn prune_finalized(&self) -> Result<(), String> {
		let is_descendent_of = sc_client_api::utils::is_descendent_of(&*self.client, None);
		let info = self.client.info();
		let mut slot_durations = self.slot_durations.write();
		let _pruned = slot_durations
			.prune(&info.finalized_hash, &info.finalized_number, &is_descendent_of, &|_| true)
			.map_err(|e| e.to_string())?;
		Ok(())
	}
}
