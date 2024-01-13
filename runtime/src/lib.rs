//! # FRAMELess Runtime
//!
//!  Welcome to the `FRAMEless` exercise, the fourth edition.
//!
//! > This assignment is based on Joshy's experiment years ago to explore building a Substrate
//! > runtime using pure Rust. If you learn something new in this exercise, attribute it to his
//! > work. We hope you to also explore new possibilities with this assignment.
//!
//! > This assignment resembles the `mini_substrate` section of the pre-course material. It is
//! > recommended to re-familiarize yourself with that if you have done it. Nonetheless, everything
//! > here is self-contained. Don't worry if you have not done `mini-substrate`.
//!
//! ## Context
//!
//! As the name suggest, this is Frame-less runtime. It is a substrate-compatible runtime, which you
//! can easily run with companion `node`, without using `frame`.
//!
//! To run the `node`, execute `cargo run -- --dev`, possibly with `--release`. `--dev` will ensure
//! that a new database is created each time, and your chain starts afresh.
//!
//! While you are welcome to explore the `node` folder, it is not part of this assignment, and you
//! can leave it as-is.
//!
//! This `node` uses a testing block-authoring/consensus scheme in which a block is produced at
//! fixed intervals. See `--consensus` cli option if you want to speed the block production up or
//! down.
//!
//! ## Assignment
//!
//! You will design a simple substrate runtime with the following properties in this assignment:
//!
//! - Only signed extrinsics are accepted, so you will learn signature verification.
//! - Basic calls for testing/learning, mainly represented in [`shared::RuntimeCall::System`].
//! - Basic currency system.
//! - Basic staking/reserving system.
//! - Nonce system, to prevent replay attacks and similar issues.
//! - Tipping, which is there to mimic transaction fee payment.
//!
//! > Given that we have no notion of inherents or unsigned extrinsics here, the words transaction
//! > and extrinsic means the same thing in this assignment.
//!
//! Read the rest of this file and `shared.rs` carefully, as it is your main specification of what
//! you have to implement.
//!
//! ### Prelude: Knowledge Recap
//!
//! #### Block Authoring and Importing
//!
//! Recall that the block author does the following:
//!
//! ```no_compile
//! (possibly call into `validate_transaction` periodically)
//!
//! Core::initialize_block(raw_header)
//! loop {
//! 		BlockBuilder::apply_extrinsic(ext)
//! }
//! BlockBuilder::finalize_block() -> final_header
//! ```
//!
//! And the block importer only calls into:
//!
//! ```no_compile
//! Core::execute_block(block)
//! ```
//!
//! We need to make sure that these two code paths each record the **correct and equal** state and
//! extrinsic root in the header. More about this in step 0. In other words, these two code paths
//! must execute the exact same logic and produce the exact same side effects.
//!
//! #### Apply vs. Dispatch
//!
//! When an extrinsic passes enough mandatory checks to justify its existence in a block, we call
//! this extrinsic to be apply-able. In other words, the extrinsic will get executed while authoring
//! and importing phase.
//!
//! Checks that must happen in apply phase are those that are mandatory to make sure a blockchain is
//! sound and safe. These include:
//!
//! - Signature verification
//! - Payment of any fees and tips.
//! - Nonce check.
//!
//! > For each of the above, take a moment to think about what happens if we don't do them. How is
//! > that blockchain vulnerable?
//!
//! Failure to meet any of these requirements means that this transaction is not even worth being in
//! the block and it should be discarded. This is what we mean by a "failed apply", and it is
//! represented `ApplyExtrinsicResult` being `Err(_)`.
//!
//! The outcome of applying can itself be a failure or success. This inner execution of the
//! extrinsic is called "dispatch".
//!
//! Contrary, a dispatch error means that the extrinsic is worth keeping in the block, but whatever
//! it wished to do may have failed. This is represented by `ApplyExtrinsicResult` being
//! `Ok(Err(_))`.
//!
//! Use this information to return the correct `ApplyExtrinsicResult` in `do_apply_extrinsic`.
//!
//!
//! #### Transaction Pool Validation
//!
//! Recall that the transaction pool can asynchronously, and at arbitrary intervals, ask the runtime
//! to validate the transactions. Two rule of thumbs about this:
//!
//! 1. The transaction pool validation must be cheap and static. As in, you MUST NOT dispatch
//!    anything in the pool validation phase.
//! 2. The transaction pool validation must contain all the checks that make a transaction
//!    apply-able.
//!
//! #### Extrinsic Format
//!
//! The extrinsic format in this assignment is defined in `shared.rs`:
#![doc = docify::embed!("src/shared.rs", Extrinsic)]
//!
//! When this extrinsic is unpacked, the runtime will first look at the signature, tip and nonce. If
//! they meet all relevant conditions, this transaction is apply-able, and should be dispatched.
//! This depends on the inner `RuntimeCall`. This type represent the different modules in your
//! runtime.
//!
//! #### Storage
//!
//! You will need to alter the runtime state in this assignment. Use `sp_io::storage` apis for this.
//! You are welcome to create more ergonomic abstractions on top of this.
//!
//! ### Step 0 - Basics
//!
//! In this section you implement all the fundamental parts of your runtime. This step is slightly
//! longer than the rest. Make sure to spend enough time on it as it it is the foundation for the
//! rest of the assignment.
//!
//! - **Proper signature check**. You need to look into `UncheckedExtrinsic.signature`, ensure it is
//!   `Some`, and only accept those that are properly signed. The signing payload should be the
//!   entire `UncheckedExtrinsic.function` (and `Extra`, which is `()`). Since `UncheckedExtrinsic`
//!   is the type that is used in all substrate-based chains, you can look at the methods and traits
//!   implemented for this type for inspiration. For example, look into `impl Checkable for
//!   UncheckedExtrinsic`.
//! - **Implement `SystemCall`**. Once you verify the signature, you can get a signer `AccountId`
//!   out of the extrinsic. Use this to implement [`shared::SystemCall`] dispatchables, such as
//!   `SetValue`.
//!
//! The above two steps should both be implemented as a part of `apply_extrinsic`.
//!
//! - **Root calculation**: Once you have successful transactions processed by your runtime you need
//!   to make sure your state and extrinsic root are correct.
//!     - After each successful `apply_extrinsic`, if the transaction passes all the checks to be
//!       apply-able, note the encoded extrinsic in `EXTRINSICS_KEY`.
//!     - In `finalize_block`, compute the extrinsics root from the above.
//!     - Flush the `EXTRINSICS_KEY` at the beginning of the next block authoring's
//!   `initialize_block`.
//!
//! The `execute_block` given to you is already complete. You can look into it and reverse-engineer
//! how to calculate roots, and what is expected of your in the block authoring part.
//!
//! In all of the above you mainly need to finish `do_apply_extrinsic` and `do_finalize_block`.
//!
//! Lastly, you need to update `validate_transaction` to make sure transactions with bad signature
//! are rejected.
//!
//! By the end of this section, you should be able to pass all the unit tests provided to you.
//!
//! Most importantly, the following test ensures that your block authoring and importing are equal
//! and correct.
#![doc = docify::embed!("src/lib.rs", import_and_author_equal)]
//!
//! Also, if you run your chain with two nodes, you will be able to test this property. Lastly, you
//! are advised to use a JSON-RPC client at this stage and try simple transactions like `SetValue`
//! and observe their effects. See the material in "Interacting with Substrate" lecture.
//!
//! #### Apply Errors
//!
//! [`sp_runtime::transaction_validity::InvalidTransaction::BadProof`] if the extrinsic has an
//! invalid signature.
//!
//! #### Transaction Pool Validation Errors
//!
//! [`sp_runtime::transaction_validity::InvalidTransaction::BadProof`] if the extrinsic has an
//! invalid signature.
//!
//! ### 1 - Currency
//!
//! Look into [`shared::CurrencyCall`] and implement the requirement as specified in the docs. Pay
//! close attention to the existential deposit section.
//!
//! ### 2 - Staking
//!
//! Look into [`shared::StakingCall`] and implement the requirement as specified in the docs.
//!
//! ### 3 - Tipping
//!
//! The ability to tip is baked into [`shared::RuntimeCallExt::tip`]. Up until this point, you were
//! expected to ignore this field.
//!
//! The tip is meant to represent transaction fees. But, to keep the assignment simple, they are
//! *optional*, therefore naming them "tip".
//!
//! If [`shared::RuntimeCallExt::tip`] is `Some(_)`, the sender's ability to pay this amount is now
//! a mandatory condition for the transaction to be apply-able. This means it must be checked in
//! both:
//!
//! * `apply_extrinsic`
//! * `validate_transaction`
//!
//! Paying the tip must not cause the account's existence to change. Specifically, an account cannot
//! be destroyed due to the tip. This is because tip payment happens prior to dispatch, and the
//! dispatch logic of the runtime assumes all accounts start at an "existing" state.
//!
//! > For example, an account that has 20 tokens cannot transfer 5 and tip 15, but it can transfer
//! > 15 and tip 5.
//!
//! All tips are transferred to [`shared::TREASURY`], as if [`shared::TREASURY`] was an account's
//! public key. That is, the account is stored under "BalancesMap", and uses the same
//! `AccountBalance`, but it can only ever have `free` balance.
//!
//! The only exception of account existence is about [`shared::TREASURY`]. If this account is
//! non-existent, it can receive tips that are smaller than `EXISTENTIAL_DEPOSIT`. If by the end of
//! the dispatch the [`shared::TREASURY`] still has less than `EXISTENTIAL_DEPOSIT`, this amount
//! should be burnt. Burning is the opposite of minting, and should update the total issuance as
//! well.
//!
//! #### Apply Errors
//!
//! [`sp_runtime::transaction_validity::InvalidTransaction::Payment`] if the extrinsic cannot pay
//! for its declared tip.
//!
//! #### Transaction Pool Validation Errors
//!
//! [`sp_runtime::transaction_validity::InvalidTransaction::Payment`] if the extrinsic cannot pay
//! for its declared tip.
//!
//! If successful, the `priority` of the transaction should be increased by the tip amount, when
//! when converted to u64. Saturating conversion should be used.
//!
//! ### Nonce
//!
//! The last field of [`shared::AccountBalance`] that has been thus far ignored is the nonce.
//!
//! Look into [`Runtime::validate_nonce`] to understand how nonce system needs to be implemented for
//! validating transaction. Additionally, nonce are verified and incremented as well as part of
//! [`Runtime::apply_predispatch`].
//!
//! > Note that unlike signature check, account existence and tipping, the behavior of
//! > `apply_extrinsic` and `validate_transaction` is noticeably different with respect to nonce.
//!
//! ## Checklist (TL;DR)
//!
//! Here's a quick summary of the major action items, in the same order as specified above.
//!
//! - [ ] (0.1) Implement signature verification in [`Runtime::do_apply_extrinsic`]. Refer
//! 	[`Runtime::verify_signed`].
//! - [ ] (0.2) Implement your first set of dispatchables in [`shared::SystemCall`] while applying
//! 	extrinsics. Refer [`Runtime::apply_dispatch`] and invoke it in [`Runtime::do_apply_extrinsic`].
//! - [ ] (0.3) Once an extrinsic passes all the `predispatch checks` while apply,
//! 	[`Runtime::note_extrinsic`] in the block.
//! - [ ] (0.4) When all extrinsics in a block has been applied, compute extrinsic root and state root
//!		in the `finalize_block`, and set it in the header. At this point all the provided unit tests
//! 	should pass.
//! - [ ] (1) Implement the [currency module](`shared::CurrencyCall`) in your runtime.
//!		- [ ] Make sure account state is always valid (`Created` or `Destroyed`).
//!     - [ ] Make sure total issuance is maintained correctly at all times.
//! - [ ] (2) Prevent replay attacks by adding a nonce system for user transactions. Refer
//! 	[`Runtime::validate_nonce`] and [`Runtime::apply_predispatch`].
//! - [ ] (3) Ability to add an optional tip while submitting a transaction. Refer
//! 	[`Runtime::validate_tip`] and [`Runtime::apply_predispatch`].
//! - [ ] (4) Build a [staking system](`shared::StakingCall`) on top of the currency module.
//!
//!
//! ## Grading
//!
//! This assignment is primarily graded through automatic tests, not by looking at the internals of
//! your runtime. Manual grading is a small part. This means you should be very careful about
//! adhering to the rules and specifications.
//!
//! Automatic Wasm grading means:
//!
//! * we do not care about the internals of your runtime, other than the standard set of runtime
//!   apis.
//! * we do not care if you derive some additional trait for some type anywhere.
//! * but we do care about your storage layout being exactly as described in [`shared`].
//! * we do care about the extrinsic format being exactly as described in [`shared`].
//! * our tests are fairly similar to `import_and_author_equal`. We construct a list of extrinsics,
//!   some of which are successful and some are not. Those that were at least apply-able are put
//!   into a block. Then, we re-import this block. Finally, we assert that authoring and importing
//!   the block had the same side effects and roots.
//!
//! While we can't force you not to change [`shared`] module, we use an exact copy of this file to
//! craft extrinsics/blocks to interact with your runtime while grading, and we expect to find the
//! types mentioned in there (eg. [`shared::AccountBalance`]) to be what we decode from your
//! storage.
//!
//! That being said, you can use types that are equivalent to their encoding to the ones mentioned
//! in [`shared`].
//!
//! Rubric: TBD.
//!
//! ## Hints
//!
//! ### Logging
//!
//! Logging can be enabled by setting the `RUST_LOG` environment variable, as such:
//!
//! ```no_compile
//! RUST_LOG=frameless=debug cargo run
//! ```
//!
//! Or equally:
//!
//! ```no_compile
//! cargo run -- --dev -l frameless=debug
//! ```
//!
//! ### Running Two Nodes
//!
//! In order to run two nodes, execute the following commands in two different terminals.
//!
//! ```no_compile
//! cargo run -- --dev --alice -l frameless=debug
//! cargo run -- --dev --bob -l frameless=debug --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<node-id-of-alice>
//! ```
//!
//! If you let the former `--alice` node progress for a bit, you will see that `--bob` will start
//! syncing from alice.
//!
//! ### Extra: `SignedExtensions`
//!
//! What we have implemented tip and nonce in this assignment as added fields to our
//! [`shared::RuntimeCallExt`], they should have ideally been implemented as a "signed extension".
//! In a separate branch, explore this, and ask for our feedback. If make progress on this front, DO
//! NOT submit it for grading, as our grading will work with the simpler `RuntimeCallExt` model.
//!
//! This is entirely optional.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

const LOG_TARGET: &'static str = "frameless";

pub mod shared;
mod solution;

use log::info;
use parity_scale_codec::{Decode, Encode};
use shared::Block;

use sp_api::impl_runtime_apis;
use sp_runtime::{
	create_runtime_str,
	generic::{self},
	traits::{BlakeTwo256, Block as BlockT},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, DispatchError,
};
use sp_std::prelude::*;

use sp_core::{hexdisplay::HexDisplay, OpaqueMetadata, H256};
use sp_runtime::traits::{Hash, Verify};
use sp_runtime::transaction_validity::{InvalidTransaction, TransactionValidityError};

#[cfg(feature = "std")]
use sp_version::NativeVersion;

use sp_version::RuntimeVersion;

use crate::shared::{
	AccountId, RuntimeCall, SystemCall, EXTRINSICS_KEY, HEADER_KEY, VALUE_KEY,
};

/// Opaque types. This is what the lectures referred to as `ClientBlock`. Notice how
/// `OpaqueExtrinsic` is merely a `Vec<u8>`.
pub mod opaque {
	use super::*;
	type OpaqueExtrinsic = sp_runtime::OpaqueExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<shared::BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, OpaqueExtrinsic>;
}

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("frameless-runtime"),
	impl_name: create_runtime_str!("frameless-runtime"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// The version information used to identify this runtime when compiled natively. This is almost
/// deprecated. Ignore.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// The main struct in this module. In frame this comes from `construct_runtime!` macro.
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct Runtime;

impl Runtime {
	#[allow(unused)]
	fn print_state() {
		let mut key = vec![];
		while let Some(next) = sp_io::storage::next_key(&key) {
			let val = sp_io::storage::get(&next).unwrap().to_vec();
			log::trace!(
				target: LOG_TARGET,
				"{} <=> {}",
				HexDisplay::from(&next),
				HexDisplay::from(&val)
			);
			key = next;
		}
	}

	fn get_state<T: Decode>(key: &[u8]) -> Option<T> {
		sp_io::storage::get(key).and_then(|d| T::decode(&mut &*d).ok())
	}

	fn mutate_state<T: Decode + Encode + Default>(key: &[u8], update: impl FnOnce(&mut T)) {
		let mut value = Self::get_state(key).unwrap_or_default();
		update(&mut value);
		sp_io::storage::set(key, &value.encode());
	}

	pub fn do_initialize_block(header: &<Block as BlockT>::Header) {
		sp_io::storage::set(&HEADER_KEY, &header.encode());
		sp_io::storage::clear(&EXTRINSICS_KEY);
	}

	pub fn do_finalize_block() -> <Block as BlockT>::Header {
		// fetch the header that was given to us at the beginning of the block.
		let header = Self::get_state::<<Block as BlockT>::Header>(HEADER_KEY)
			.expect("We initialized with header, it never got mutated, qed");

		// and make sure to _remove_ it.
		sp_io::storage::clear(&HEADER_KEY);

		Runtime::print_state();
		let header = Self::solution_finalize_block(header);

		header
	}

	/// Apply a single extrinsic.
	///
	/// In our template, we call into this from both block authoring, and block import.
	pub fn do_apply_extrinsic(ext: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
		// Self::solution_apply_extrinsic(ext.clone())

		let signer = Self::verify_signed(ext.clone())?;
		Self::apply_predispatch(&ext, signer)?;
		let dispatch_outcome = Self::apply_dispatch(&ext, signer);
		Self::note_extrinsic(&ext);
		Ok(dispatch_outcome)
	}

	/// Your code path to execute a block that has been previously authored.
	pub fn do_execute_block(block: Block) {
		// clear any previous extrinsics. data.
		// NOTE: Look into FRAME, namely the system and executive crates and see if this is any
		// different in FRAME?
		sp_io::storage::clear(&EXTRINSICS_KEY);

		for extrinsic in block.clone().extrinsics {
			let _outcome = Runtime::do_apply_extrinsic(extrinsic)
				.expect("A block author has provided us with an invalid block; bailing; qed");
		}

		// check state root. Clean the state prior to asking for the root.
		sp_io::storage::clear(&HEADER_KEY);

		Self::print_state();

		// NOTE: if we forget to do this, how can you mess with the blockchain?
		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = H256::decode(&mut &raw_state_root[..]).unwrap();
		assert_eq!(block.header.state_root, state_root, "state root mismatch!");

		// check extrinsics root
		let extrinsics = Self::get_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY).unwrap_or_default();
		let extrinsics_root =
			BlakeTwo256::ordered_trie_root(extrinsics, sp_runtime::StateVersion::V0);
		assert_eq!(block.header.extrinsics_root, extrinsics_root);

		info!(target: LOG_TARGET, "Finishing block import.");
	}

	/// Your transaction pool validation.
	pub fn do_validate_transaction(
		_source: TransactionSource,
		ext: <Block as BlockT>::Extrinsic,
		_block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		let signer = Self::verify_signed(ext.clone())?;
		let valid = Self::validate_nonce(&ext, signer)?;
		Self::validate_tip(&ext, signer)?;

		Ok(valid)
		// Self::solution_validate_transaction(_source, ext, _block_hash)
	}

	/// if you want some initial state in your own local test (when you actually run the node with
	/// `cargo run`), then add them here. We don't ever call into this API.
	pub fn do_build_config() -> sp_genesis_builder::Result {
		Self::solution_do_build_config()
	}

	/// Verify the extrinsic is properly signed and return the signing account if successful.
	///
	/// #### Errors
	///
	/// If no `extrinsic.signature` is present or if signature is not valid, return the error
	/// [`InvalidTransaction::BadProof`].
	fn verify_signed(
		extrinsic: <Block as BlockT>::Extrinsic,
	) -> Result<AccountId, TransactionValidityError> {
		// todo!()

		if let Some((address, signature, _)) = &extrinsic.signature {
			let payload = (extrinsic.function.clone(), ()).encode();
			return signature
				.verify(payload.as_ref(), address)
				.then(|| address.clone())
				.ok_or(TransactionValidityError::Invalid(InvalidTransaction::BadProof));
		}

		Err(TransactionValidityError::Invalid(InvalidTransaction::BadProof))
	}

	/// Perform the predispatch checks and tasks, namely (1) check and update nonce, and (2) collect
	/// tip from the signer account and transfer it to the treasury account. If any of these actions
	/// fail, no changes should be made to the state and the correct error is returned.
	///
	/// ## Nonce
	///
	/// You need to check nonce supplied in the extrinsic against the nonce stored in the signer's
	/// account.
	///
	/// #### Errors
	///
	/// [`Future`](`sp_runtime::transaction_validity::InvalidTransaction::Future`) or
	/// [`Stale`](sp_runtime::transaction_validity::InvalidTransaction::Stale) if the transaction's
	/// nonce is not correct.
	///
	/// ## Collect Tip
	///
	/// Collect tip if [`shared::RuntimeCallExt::tip`] is `Some(_)`. The tip is transferred to
	/// treasury.
	///
	/// IMPORTANT: If paying the tip causes the signer's account existence to change (read more
	/// about this in `ExistentialDeposit` section of [`shared::CurrencyCall`]), the predispatch
	/// should fail and revert any changes it made (such as incrementing the Nonce).
	///
	/// #### Errors
	///
	/// [`sp_runtime::transaction_validity::InvalidTransaction::Payment`] if the extrinsic cannot
	/// pay for its declared tip.
	fn apply_predispatch(
		extrinsic: &<Block as BlockT>::Extrinsic,
		signer: AccountId,
	) -> Result<(), TransactionValidityError> {
		todo!()
	}

	/// TODO: Look at the `UncheckedExtrinsic.function.call` and dispatch it to the corresponding
	/// variant of [`shared::RuntimeCall`].
	fn apply_dispatch(
		extrinsic: &<Block as BlockT>::Extrinsic,
		signer: AccountId,
	) -> Result<(), DispatchError> {
		match extrinsic.clone().function.call {
			RuntimeCall::System(SystemCall::Set { value }) => {
				sp_io::storage::set(VALUE_KEY, &value.encode());
				Ok(())
			},
			_ => Err(DispatchError::Unavailable),
		}
	}

	/// ## Nonce validation
	///
	/// You should implement a nonce system, as explained as a part of the tx-pool lecture. In
	/// short, the validation of each transaction should `require` nonce `(sender, n-1).encode()`
	/// and provide `(sender, n).encode()`. All accounts are created with nonce 0. The first valid
	/// transaction nonce is 0, which if successful, will set the account nonce to 1.
	///
	/// #### Errors
	///
	/// [`sp_runtime::transaction_validity::InvalidTransaction::Future`] or `Stale` if the
	/// transaction's nonce is not correct.
	///
	/// If valid, the correct `provides` and `requires` should be set.
	fn validate_nonce(
		ext: &<Block as BlockT>::Extrinsic,
		signer: AccountId,
	) -> TransactionValidity {
		todo!()
	}

	/// Verify that the sender is able to pay the tip without their accounts ending up `Destroyed`
	/// or in `Invalid` state.
	fn validate_tip(ext: &<Block as BlockT>::Extrinsic, signer: AccountId) -> TransactionValidity {
		todo!()
	}

	/// Note extrinsic in the current block.
	///
	/// Should be noted in the state at the end of applying an extrinsic if it passes all
	/// predispatch checks. They are flushed at the beginning of the next block.
	fn note_extrinsic(ext: &<Block as BlockT>::Extrinsic) {
		Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
			current.push(ext.encode());
		});
	}
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			info!(
				target: LOG_TARGET,
				"Entering execute_block block: {:?} (exts: {})",
				block,
				block.extrinsics.len()
			);
			Self::do_execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			info!(
				target: LOG_TARGET,
				"Entering initialize_block. header: {:?} / version: {:?}", header, VERSION.spec_version
			);
			Self::do_initialize_block(header)
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			info!(target: LOG_TARGET, "Entering apply_extrinsic: {:?}", extrinsic);
			Self::do_apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			let header = Self::do_finalize_block();
			info!(target: LOG_TARGET, "Finalized block authoring {:?}", header);
			header
		}

		fn inherent_extrinsics(_data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			Default::default()
		}

		fn check_inherents(
			_block: Block,
			_data: sp_inherents::InherentData
		) -> sp_inherents::CheckInherentsResult {
			Default::default()
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			log::debug!(target: LOG_TARGET,"Entering validate_transaction. tx: {:?}", tx);
			Self::do_validate_transaction(source, tx, block_hash)
		}
	}

	// You can safely ignore everything after this.
	impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
		fn create_default_config() -> Vec<u8> {
			// ignore this.
			let genesis = serde_json::json!({});
			serde_json::to_string(&genesis)
				.expect("genesis state should be convertible to json")
				.into_bytes()
		}

		fn build_config(_config: Vec<u8>) -> sp_genesis_builder::Result {
			Runtime::do_build_config()
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Default::default())
		}

		fn metadata_at_version(_version: u32) -> Option<OpaqueMetadata> {
			Default::default()
		}

		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Default::default()
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(_header: &<Block as BlockT>::Header) {}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}

		fn decode_session_keys(
			_: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			Default::default()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shared::{AccountId, RuntimeCallExt};
	use parity_scale_codec::Encode;
	use shared::{Extrinsic, RuntimeCall, VALUE_KEY};
	use sp_core::hexdisplay::HexDisplay;
	use sp_io::TestExternalities;
	use sp_runtime::{
		traits::Extrinsic as _,
		transaction_validity::{InvalidTransaction, TransactionValidityError},
	};

	fn set_value_call(value: u32, nonce: u32) -> RuntimeCallExt {
		RuntimeCallExt {
			call: RuntimeCall::System(shared::SystemCall::Set { value }),
			tip: None,
			nonce,
		}
	}

	fn unsigned_set_value(value: u32) -> Extrinsic {
		let call = RuntimeCallExt {
			call: RuntimeCall::System(shared::SystemCall::Set { value }),
			tip: None,
			nonce: 0,
		};
		Extrinsic::new(call, None).unwrap()
	}

	fn signed_set_value(value: u32, nonce: u32) -> (Extrinsic, AccountId) {
		let call = set_value_call(value, nonce);
		let signer = sp_keyring::AccountKeyring::Alice;
		let payload = call.encode();
		let signature = signer.sign(&payload);
		(Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap(), signer.public())
	}

	/// Return the list of extrinsics that are noted in the `EXTRINSICS_KEY`.
	fn noted_extrinsics() -> Vec<Vec<u8>> {
		sp_io::storage::get(EXTRINSICS_KEY)
			.and_then(|bytes| <Vec<Vec<u8>> as Decode>::decode(&mut &*bytes).ok())
			.unwrap_or_default()
	}

	/// Fund an account with the `EXISTENTIAL_DEPOSIT` such that it can transact. This can be empty
	/// for now, but once you implement the currency part, you need to use it.
	fn fund_account(who: AccountId) {
		solution::fund_account(who)
	}

	#[test]
	fn does_it_print() {
		// runt this with `cargo test does_it_print -- --nocapture`
		println!("Something");
	}

	#[test]
	fn does_it_log() {
		// run this with RUST_LOG=frameless=trace cargo test -p runtime does_it_log
		sp_tracing::try_init_simple();
		log::info!(target: LOG_TARGET, "Something");
	}

	#[docify::export]
	#[test]
	fn host_function_call_works() {
		// this is just to demonstrate to you that you should always wrap any code containing host
		// functions in `TestExternalities`.
		TestExternalities::new_empty().execute_with(|| {
			sp_io::storage::get(&VALUE_KEY);
		})
	}

	#[docify::export]
	#[test]
	fn encode_examples() {
		// demonstrate some basic encodings. Example usage:
		//
		// ```
		// wscat -c 127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"state_getStorage", "params": ["0x123"]}'
		// wscat -c ws://127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"author_submitExtrinsic", "params": ["0x123"]}'
		// ```
		let unsigned = Extrinsic::new_unsigned(set_value_call(42, 0));

		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42, 0);
		let payload = (call).encode();
		let signature = signer.sign(&payload);
		let signed = Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap();

		println!("unsigned = {:?} {:?}", unsigned, HexDisplay::from(&unsigned.encode()));
		println!("signed {:?} {:?}", signed, HexDisplay::from(&signed.encode()));
		println!("value key = {:?}", HexDisplay::from(&VALUE_KEY));
	}

	#[docify::export]
	#[test]
	fn signed_set_value_works() {
		// A signed `Set` works.
		let (ext, who) = signed_set_value(42, 0);
		TestExternalities::new_empty().execute_with(|| {
			fund_account(who);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(noted_extrinsics().len(), 0);

			Runtime::do_apply_extrinsic(ext).unwrap().unwrap();

			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), Some(42));
			assert_eq!(noted_extrinsics().len(), 1, "transaction should have been noted!");
		});
	}

	#[docify::export]
	#[test]
	fn bad_signature_fails() {
		// A poorly signed extrinsic must fail.
		let signer = sp_keyring::AccountKeyring::Alice;
		let call = set_value_call(42, 0);
		let bad_call = set_value_call(43, 0);
		let payload = (bad_call).encode();
		let signature = signer.sign(&payload);
		let ext = Extrinsic::new(call, Some((signer.public(), signature, ()))).unwrap();

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(
				Runtime::do_apply_extrinsic(ext).unwrap_err(),
				TransactionValidityError::Invalid(InvalidTransaction::BadProof)
			);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(noted_extrinsics().len(), 0, "transaction should have not been noted!");
		});
	}

	#[docify::export]
	#[test]
	fn unsigned_set_value_does_not_work() {
		// An unsigned `Set` must fail as well.
		let ext = unsigned_set_value(42);

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(
				Runtime::do_apply_extrinsic(ext).unwrap_err(),
				TransactionValidityError::Invalid(InvalidTransaction::BadProof)
			);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(noted_extrinsics().len(), 0);
		});
	}

	#[docify::export]
	#[test]
	fn validate_works() {
		// An unsigned `Set` cannot be validated. Same should go for one with a bad signature.
		let ext = unsigned_set_value(42);

		TestExternalities::new_empty().execute_with(|| {
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
			assert_eq!(
				Runtime::do_validate_transaction(
					TransactionSource::External,
					ext,
					Default::default()
				)
				.unwrap_err(),
				TransactionValidityError::Invalid(InvalidTransaction::BadProof)
			);
			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), None);
		});
	}

	#[docify::export]
	#[test]
	fn import_and_author_equal() {
		// a few dummy extrinsics.
		let (ext1, _) = signed_set_value(42, 0);
		let (ext2, _) = signed_set_value(43, 1);
		let (ext3, who) = signed_set_value(44, 2);
		let ext4 = unsigned_set_value(3);

		let header = shared::Header {
			digest: Default::default(),
			extrinsics_root: Default::default(),
			parent_hash: Default::default(),
			number: 0,
			state_root: Default::default(),
		};

		// authoring a block:
		let block = TestExternalities::new_empty().execute_with(|| {
			fund_account(who);
			Runtime::do_initialize_block(&header);
			drop(header);

			Runtime::do_apply_extrinsic(ext1.clone()).unwrap().unwrap();
			Runtime::do_apply_extrinsic(ext2.clone()).unwrap().unwrap();
			Runtime::do_apply_extrinsic(ext3.clone()).unwrap().unwrap();
			let _ = Runtime::do_apply_extrinsic(ext4.clone()).unwrap_err();

			let header = Runtime::do_finalize_block();

			assert!(
				sp_io::storage::get(HEADER_KEY).is_none(),
				"header must have been cleared from storage"
			);
			let extrinsics = noted_extrinsics();
			assert_eq!(extrinsics.len(), 3, "incorrect extrinsics_key recorded in state");

			let expected_state_root = {
				let raw_state_root = &sp_io::storage::root(Default::default())[..];
				H256::decode(&mut &raw_state_root[..]).unwrap()
			};
			let expected_extrinsics_root =
				BlakeTwo256::ordered_trie_root(extrinsics, sp_runtime::StateVersion::V0);

			assert_eq!(
				header.state_root, expected_state_root,
				"block finalization should set correct state root in header"
			);
			assert_eq!(
				header.extrinsics_root, expected_extrinsics_root,
				"block finalization should set correct extrinsics root in header"
			);

			Block { extrinsics: vec![ext1, ext2, ext3], header }
		});

		// now re-importing it.
		TestExternalities::new_empty().execute_with(|| {
			fund_account(who);
			// This should internally check state/extrinsics root. If it does not panic, then we are
			// gucci.
			Runtime::do_execute_block(block.clone());

			assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), Some(44));

			// double check the extrinsic and state root:
			assert_eq!(
				block.header.state_root,
				H256::decode(&mut &sp_io::storage::root(Default::default())[..][..]).unwrap(),
				"incorrect state root in authored block after importing"
			);
			assert_eq!(
				block.header.extrinsics_root,
				BlakeTwo256::ordered_trie_root(
					block.extrinsics.into_iter().map(|e| e.encode()).collect::<Vec<_>>(),
					sp_runtime::StateVersion::V0
				),
				"incorrect extrinsics root in authored block",
			);
		});
	}
}
