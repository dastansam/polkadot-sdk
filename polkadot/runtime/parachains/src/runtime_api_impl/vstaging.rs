// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Put implementations of functions from staging APIs here.

use crate::{configuration, initializer, scheduler, shared};
use primitives::{
	vstaging::{ApprovalVotingParams, NodeFeatures},
	CoreIndex, Id as ParaId, ValidatorIndex,
};
use sp_std::{
	collections::{btree_map::BTreeMap, vec_deque::VecDeque},
	prelude::Vec,
};

/// Implementation for `DisabledValidators`
// CAVEAT: this should only be called on the node side
// as it might produce incorrect results on session boundaries
pub fn disabled_validators<T>() -> Vec<ValidatorIndex>
where
	T: shared::Config,
{
	<shared::Pallet<T>>::disabled_validators()
}

/// Returns the current state of the node features.
pub fn node_features<T: initializer::Config>() -> NodeFeatures {
	<configuration::Pallet<T>>::config().node_features
}

/// Approval voting subsystem configuration parameters
pub fn approval_voting_params<T: initializer::Config>() -> ApprovalVotingParams {
	let config = <configuration::Pallet<T>>::config();
	config.approval_voting_params
}

/// Returns the claimqueue from the scheduler
pub fn claim_queue<T: scheduler::Config>() -> BTreeMap<CoreIndex, VecDeque<ParaId>> {
	<scheduler::Pallet<T>>::claimqueue()
		.into_iter()
		.map(|(core_index, entries)| {
			(core_index, entries.into_iter().map(|e| e.para_id()).collect())
		})
		.collect()
}
