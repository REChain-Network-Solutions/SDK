#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::{DispatchResult, DispatchResultWithPostInfo}, pallet_prelude::*, traits::fungible};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use sp_runtime::traits::{CheckedAdd, CheckedSub, Zero};
	use sp_arithmetic::fixed_point::FixedU128;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Native currency for governance deposits
		type Currency: fungible::Inspect<Self::AccountId> + fungible::Mutate<Self::AccountId>;

		/// Maximum length of proposal title
		#[pallet::constant]
		type MaxProposalTitleLength: Get<u32>;

		/// Maximum length of proposal description
		#[pallet::constant]
		type MaxProposalDescriptionLength: Get<u32>;

		/// Minimum deposit for proposal
		#[pallet::constant]
		type MinimumProposalDeposit: Get<u128>;

		/// Voting period for proposals
		#[pallet::constant]
		type VotingPeriod: Get<Self::BlockNumber>;

		/// Enactment delay for approved proposals
		#[pallet::constant]
		type EnactmentDelay: Get<Self::BlockNumber>;

		/// Maximum council members
		#[pallet::constant]
		type MaxCouncilMembers: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Governance proposals storage
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // Proposal ID
		Proposal<T>,
		OptionQuery,
	>;

	/// Proposal votes storage
	#[pallet::storage]
	#[pallet::getter(fn proposal_votes)]
	pub type ProposalVotes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u64, // Proposal ID
		Blake2_128Concat,
		T::AccountId,
		Vote,
		OptionQuery,
	>;

	/// Council members storage
	#[pallet::storage]
	#[pallet::getter(fn council_members)]
	pub type CouncilMembers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		CouncilMember<T>,
		OptionQuery,
	>;

	/// Treasury proposals storage
	#[pallet::storage]
	#[pallet::getter(fn treasury_proposals)]
	pub type TreasuryProposals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // Treasury Proposal ID
		TreasuryProposal<T>,
		OptionQuery,
	>;

	/// Treasury balance
	#[pallet::storage]
	#[pallet::getter(fn treasury_balance)]
	pub type TreasuryBalance<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// Next proposal ID counter
	#[pallet::storage]
	#[pallet::getter(fn next_proposal_id)]
	pub type NextProposalId<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Next treasury proposal ID counter
	#[pallet::storage]
	#[pallet::getter(fn next_treasury_proposal_id)]
	pub type NextTreasuryProposalId<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Proposal information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Proposal<T: Config> {
		/// Proposal ID
		pub id: u64,
		/// Proposal title
		pub title: BoundedVec<u8, T::MaxProposalTitleLength>,
		/// Proposal description
		pub description: BoundedVec<u8, T::MaxProposalDescriptionLength>,
		/// Proposer
		pub proposer: T::AccountId,
		/// Proposal deposit
		pub deposit: u128,
		/// Proposal status
		pub status: ProposalStatus,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Voting end timestamp
		pub voting_end: T::BlockNumber,
		/// Enactment timestamp
		pub enactment: T::BlockNumber,
		/// Vote counts
		pub votes: VoteCounts,
		/// Proposal metadata hash
		pub metadata_hash: Option<BoundedVec<u8, T::MaxProposalDescriptionLength>>,
	}

	/// Vote information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Vote {
		/// Voter account
		pub voter: T::AccountId,
		/// Vote direction
		pub direction: VoteDirection,
		/// Vote weight
		pub weight: u128,
		/// Vote timestamp
		pub timestamp: T::BlockNumber,
	}

	/// Vote direction
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum VoteDirection {
		/// Aye (yes)
		Aye,
		/// Nay (no)
		Nay,
		/// Abstain
		Abstain,
	}

	/// Vote counts
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct VoteCounts {
		/// Aye votes
		pub aye: u128,
		/// Nay votes
		pub nay: u128,
		/// Abstain votes
		pub abstain: u128,
		/// Total voter count
		pub voter_count: u32,
	}

	/// Proposal status
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ProposalStatus {
		/// Proposal is pending
		Pending,
		/// Proposal is in voting period
		Active,
		/// Proposal passed and is being enacted
		Succeeded,
		/// Proposal failed
		Failed,
		/// Proposal was cancelled
		Cancelled,
		/// Proposal was enacted
		Enacted,
	}

	/// Council member information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CouncilMember<T: Config> {
		/// Member account
		pub account: T::AccountId,
		/// Member name
		pub name: BoundedVec<u8, T::MaxProposalTitleLength>,
		/// Member role
		pub role: BoundedVec<u8, T::MaxProposalTitleLength>,
		/// Member since
		pub member_since: T::BlockNumber,
		/// Active status
		pub active: bool,
		/// Proposal count
		pub proposal_count: u32,
	}

	/// Treasury proposal information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct TreasuryProposal<T: Config> {
		/// Proposal ID
		pub id: u64,
		/// Proposer
		pub proposer: T::AccountId,
		/// Beneficiary
		pub beneficiary: T::AccountId,
		/// Requested amount
		pub amount: u128,
		/// Proposal description
		pub description: BoundedVec<u8, T::MaxProposalDescriptionLength>,
		/// Proposal status
		pub status: TreasuryProposalStatus,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Approval count
		pub approval_count: u32,
	}

	/// Treasury proposal status
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum TreasuryProposalStatus {
		/// Proposal pending council approval
		Pending,
		/// Proposal approved and funded
		Approved,
		/// Proposal rejected
		Rejected,
		/// Funds spent
		Spent,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New governance proposal created
		ProposalCreated {
			proposal_id: u64,
			proposer: T::AccountId,
			title: BoundedVec<u8, T::MaxProposalTitleLength>,
		},
		/// Vote cast on proposal
		VoteCast {
			proposal_id: u64,
			voter: T::AccountId,
			direction: VoteDirection,
			weight: u128,
		},
		/// Proposal status changed
		ProposalStatusChanged {
			proposal_id: u64,
			old_status: ProposalStatus,
			new_status: ProposalStatus,
		},
		/// Council member added
		CouncilMemberAdded {
			member: T::AccountId,
			name: BoundedVec<u8, T::MaxProposalTitleLength>,
		},
		/// Council member removed
		CouncilMemberRemoved {
			member: T::AccountId,
		},
		/// Treasury proposal created
		TreasuryProposalCreated {
			proposal_id: u64,
			proposer: T::AccountId,
			beneficiary: T::AccountId,
			amount: u128,
		},
		/// Treasury proposal approved
		TreasuryProposalApproved {
			proposal_id: u64,
			amount: u128,
			beneficiary: T::AccountId,
		},
		/// Treasury funds spent
		TreasurySpent {
			proposal_id: u64,
			amount: u128,
			beneficiary: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proposal already exists
		ProposalAlreadyExists,
		/// Proposal not found
		ProposalNotFound,
		/// Insufficient deposit for proposal
		InsufficientDeposit,
		/// Voting period ended
		VotingPeriodEnded,
		/// Already voted on proposal
		AlreadyVoted,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Proposal title too long
		ProposalTitleTooLong,
		/// Proposal description too long
		ProposalDescriptionTooLong,
		/// Council is full
		CouncilFull,
		/// Council member not found
		CouncilMemberNotFound,
		/// Treasury proposal not found
		TreasuryProposalNotFound,
		/// Insufficient treasury balance
		InsufficientTreasuryBalance,
		/// Invalid proposal status
		InvalidProposalStatus,
		/// Council vote required
		CouncilVoteRequired,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create new governance proposal
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_proposal(
			origin: OriginFor<T>,
			title: Vec<u8>,
			description: Vec<u8>,
			metadata_hash: Option<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let title = BoundedVec::<u8, T::MaxProposalTitleLength>::try_from(title.clone())
				.map_err(|_| Error::<T>::ProposalTitleTooLong)?;
			let description = BoundedVec::<u8, T::MaxProposalDescriptionLength>::try_from(description)
				.map_err(|_| Error::<T>::ProposalDescriptionTooLong)?;
			let metadata_hash = if let Some(hash) = metadata_hash {
				Some(BoundedVec::<u8, T::MaxProposalDescriptionLength>::try_from(hash)
					.map_err(|_| Error::<T>::ProposalDescriptionTooLong)?)
			} else {
				None
			};

			// Check minimum deposit
			ensure!(T::Currency::reducible_balance(&who, false) >= T::MinimumProposalDeposit::get(), Error::<T>::InsufficientDeposit);

			// Get next proposal ID
			let proposal_id = NextProposalId::<T>::get();
			let next_id = proposal_id.checked_add(1).ok_or(Error::<T>::ProposalAlreadyExists)?;

			// Create proposal
			let proposal = Proposal::<T> {
				id: proposal_id,
				title: title.clone(),
				description,
				proposer: who.clone(),
				deposit: T::MinimumProposalDeposit::get(),
				status: ProposalStatus::Pending,
				created: frame_system::Pallet::<T>::block_number(),
				voting_end: frame_system::Pallet::<T>::block_number() + T::VotingPeriod::get(),
				enactment: frame_system::Pallet::<T>::block_number() + T::VotingPeriod::get() + T::EnactmentDelay::get(),
				votes: VoteCounts {
					aye: 0,
					nay: 0,
					abstain: 0,
					voter_count: 0,
				},
				metadata_hash,
			};

			// Store proposal
			Proposals::<T>::insert(proposal_id, proposal);
			NextProposalId::<T>::set(next_id);

			// Reserve deposit
			T::Currency::reserve(&who, T::MinimumProposalDeposit::get())?;

			// Emit event
			Self::deposit_event(Event::ProposalCreated {
				proposal_id,
				proposer: who,
				title: title.clone(),
			});

			Ok(())
		}

		/// Vote on governance proposal
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn vote_on_proposal(
			origin: OriginFor<T>,
			proposal_id: u64,
			direction: VoteDirection,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get proposal
			let mut proposal = Proposals::<T>::get(proposal_id)
				.ok_or(Error::<T>::ProposalNotFound)?;

			// Check if voting period is active
			let current_block = frame_system::Pallet::<T>::block_number();
			ensure!(current_block <= proposal.voting_end, Error::<T>::VotingPeriodEnded);

			// Check if already voted
			ensure!(!ProposalVotes::<T>::contains_key(proposal_id, &who), Error::<T>::AlreadyVoted);

			// Calculate vote weight (could be based on stake, reputation, etc.)
			let vote_weight = T::Currency::balance(&who);

			// Create vote
			let vote = Vote {
				voter: who.clone(),
				direction: direction.clone(),
				weight: vote_weight,
				timestamp: current_block,
			};

			// Update vote counts
			match direction {
				VoteDirection::Aye => {
					proposal.votes.aye = proposal.votes.aye.checked_add(vote_weight)
						.ok_or(Error::<T>::InvalidAmount)?;
				},
				VoteDirection::Nay => {
					proposal.votes.nay = proposal.votes.nay.checked_add(vote_weight)
						.ok_or(Error::<T>::InvalidAmount)?;
				},
				VoteDirection::Abstain => {
					proposal.votes.abstain = proposal.votes.abstain.checked_add(vote_weight)
						.ok_or(Error::<T>::InvalidAmount)?;
				},
			}

			proposal.votes.voter_count += 1;

			// Update proposal status if voting ended
			if current_block > proposal.voting_end {
				if proposal.votes.aye > proposal.votes.nay {
					proposal.status = ProposalStatus::Succeeded;
				} else {
					proposal.status = ProposalStatus::Failed;
				}
			}

			// Store vote and updated proposal
			ProposalVotes::<T>::insert(proposal_id, &who, vote);
			Proposals::<T>::insert(proposal_id, proposal);

			// Emit event
			Self::deposit_event(Event::VoteCast {
				proposal_id,
				voter: who,
				direction,
				weight: vote_weight,
			});

			Ok(())
		}

		/// Add council member
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn add_council_member(
			origin: OriginFor<T>,
			member: T::AccountId,
			name: Vec<u8>,
			role: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let name = BoundedVec::<u8, T::MaxProposalTitleLength>::try_from(name)
				.map_err(|_| Error::<T>::ProposalTitleTooLong)?;
			let role = BoundedVec::<u8, T::MaxProposalTitleLength>::try_from(role)
				.map_err(|_| Error::<T>::ProposalTitleTooLong)?;

			// Check council capacity
			let member_count = CouncilMembers::<T>::iter().count() as u32;
			ensure!(member_count < T::MaxCouncilMembers::get(), Error::<T>::CouncilFull);

			// Check if already a member
			ensure!(!CouncilMembers::<T>::contains_key(&member), Error::<T>::CouncilMemberNotFound);

			// Create council member
			let council_member = CouncilMember::<T> {
				account: member.clone(),
				name: name.clone(),
				role: role.clone(),
				member_since: frame_system::Pallet::<T>::block_number(),
				active: true,
				proposal_count: 0,
			};

			// Store council member
			CouncilMembers::<T>::insert(&member, council_member);

			// Emit event
			Self::deposit_event(Event::CouncilMemberAdded {
				member: member.clone(),
				name: name.clone(),
			});

			Ok(())
		}

		/// Create treasury proposal
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_treasury_proposal(
			origin: OriginFor<T>,
			beneficiary: T::AccountId,
			amount: u128,
			description: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let description = BoundedVec::<u8, T::MaxProposalDescriptionLength>::try_from(description)
				.map_err(|_| Error::<T>::ProposalDescriptionTooLong)?;

			// Check treasury balance
			let treasury_balance = TreasuryBalance::<T>::get();
			ensure!(treasury_balance >= amount, Error::<T>::InsufficientTreasuryBalance);

			// Get next treasury proposal ID
			let treasury_proposal_id = NextTreasuryProposalId::<T>::get();
			let next_treasury_id = treasury_proposal_id.checked_add(1).ok_or(Error::<T>::ProposalAlreadyExists)?;

			// Create treasury proposal
			let treasury_proposal = TreasuryProposal::<T> {
				id: treasury_proposal_id,
				proposer: who.clone(),
				beneficiary: beneficiary.clone(),
				amount,
				description: description.clone(),
				status: TreasuryProposalStatus::Pending,
				created: frame_system::Pallet::<T>::block_number(),
				approval_count: 0,
			};

			// Store treasury proposal
			TreasuryProposals::<T>::insert(treasury_proposal_id, treasury_proposal);
			NextTreasuryProposalId::<T>::set(next_treasury_id);

			// Emit event
			Self::deposit_event(Event::TreasuryProposalCreated {
				proposal_id: treasury_proposal_id,
				proposer: who,
				beneficiary: beneficiary.clone(),
				amount,
			});

			Ok(())
		}

		/// Approve treasury proposal (council only)
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn approve_treasury_proposal(
			origin: OriginFor<T>,
			treasury_proposal_id: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if caller is council member
			ensure!(CouncilMembers::<T>::contains_key(&who), Error::<T>::NotAuthorized);

			// Get treasury proposal
			let mut treasury_proposal = TreasuryProposals::<T>::get(treasury_proposal_id)
				.ok_or(Error::<T>::TreasuryProposalNotFound)?;

			// Check proposal status
			ensure!(treasury_proposal.status == TreasuryProposalStatus::Pending, Error::<T>::InvalidProposalStatus);

			// Check treasury balance
			let treasury_balance = TreasuryBalance::<T>::get();
			ensure!(treasury_balance >= treasury_proposal.amount, Error::<T>::InsufficientTreasuryBalance);

			// Update proposal status
			treasury_proposal.status = TreasuryProposalStatus::Approved;
			treasury_proposal.approval_count += 1;

			// Transfer funds from treasury to beneficiary
			let new_treasury_balance = treasury_balance.checked_sub(treasury_proposal.amount)
				.ok_or(Error::<T>::InsufficientTreasuryBalance)?;
			TreasuryBalance::<T>::set(new_treasury_balance);

			// Transfer to beneficiary
			T::Currency::mint_into(&treasury_proposal.beneficiary, treasury_proposal.amount)?;

			// Store updated proposal
			TreasuryProposals::<T>::insert(treasury_proposal_id, treasury_proposal);

			// Emit events
			Self::deposit_event(Event::TreasuryProposalApproved {
				proposal_id: treasury_proposal_id,
				amount: treasury_proposal.amount,
				beneficiary: treasury_proposal.beneficiary.clone(),
			});

			Self::deposit_event(Event::TreasurySpent {
				proposal_id: treasury_proposal_id,
				amount: treasury_proposal.amount,
				beneficiary: treasury_proposal.beneficiary,
			});

			Ok(())
		}

		/// Execute approved proposal
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn execute_proposal(
			origin: OriginFor<T>,
			proposal_id: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get proposal
			let mut proposal = Proposals::<T>::get(proposal_id)
				.ok_or(Error::<T>::ProposalNotFound)?;

			// Check if caller is authorized (council member)
			ensure!(CouncilMembers::<T>::contains_key(&who), Error::<T>::NotAuthorized);

			// Check if proposal succeeded and is ready for enactment
			let current_block = frame_system::Pallet::<T>::block_number();
			ensure!(proposal.status == ProposalStatus::Succeeded, Error::<T>::InvalidProposalStatus);
			ensure!(current_block >= proposal.enactment, Error::<T>::VotingPeriodEnded);

			// Mark proposal as enacted
			proposal.status = ProposalStatus::Enacted;

			// Store updated proposal
			Proposals::<T>::insert(proposal_id, proposal);

			// Return deposit to proposer
			T::Currency::unreserve(&proposal.proposer, proposal.deposit);

			// Emit event
			Self::deposit_event(Event::ProposalStatusChanged {
				proposal_id,
				old_status: ProposalStatus::Succeeded,
				new_status: ProposalStatus::Enacted,
			});

			Ok(())
		}
	}
}