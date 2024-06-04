//! # FRAMELess Runtime
//!
//! Welcome to the FRAMELess assignment.
//!
//! > This assignment is based on Joshy's experiment years ago to explore building a Substrate
//! > runtime using pure Rust. If you learn something new in this exercise, attribute it to his
//! > work.
//!
//! The code in the rust crate is a basic Substrate-but-not-FRAME-based-runtime. It uses only crates
//! from the substrate primitives, and the final outcome of it is a fully functioning
//! Substrate-based runtime. How cool is that?
//!
//! This assignment is not graded, and therefore does not have one strict set of requirements. We
//! provide you with a list of suggestions, and you do as you prefer. Try and maximize your learning
//! while working with this template. From experience, we know that one of the unique things
//! students learn is PBA is "how things work under the hood", and this template is indeed one of
//! those.
//!
//! ## Suggestions
//!
//! * Understand the code. This will be done in a walkthrough in the class.
//! * Run tests and understand what each one is testing.
//!
//! > In these two steps, you should pay close attention to why/how the state root is being kept
//! > correctly. Moreover, you will know which runtime-apis are used at which steps.
//!
//!
//! * Run this runtime. For this assignment, we only use a custom node that is independent of your
//!   runtime. We call such nodes an `omni-node`.
//! * Work with RPCs and your runtime. Read existing state (`:code`). Submit a simple
//!   `Call::SetValue`.
//! * Work with `chain-spec-builder` to create a chain spec for your runtime, use an existing
//!   preset, or generate default + patch.
//!
//! Finally, feel free then to extend the runtime with more application logic. You have two general
//! ways:
//!
//! 1. Add a mapping of account-ids and balances, and implement a simple transfer function.
//! 2. Add proper signature verification to the extrinsics.
//!
//! ## How to Run
//!
//! You can run this runtime with a custom minimal `omni-node` that is prepared for PBA.
//!
//! First:
//!
//! ```text
//! cargo install --force --git https://github.com/kianenigma/pba-omni-node
//! ```
//!
//! Then, you can run:
//!
//! ```text
//! # build the wasm runtime, possibly with log targets.
//! RUST_LOG=frameless=info cargo build --release
//! # pass it to the node
//! RUST_LOG=frameless=debug
//! pba-omni-node
//! 	# the path to the runtime.
//! 	--runtime ./target/release/wbuild/runtime/runtime.wasm \
//! 	# ensures we spin up a new database each time.
//! 	--tmp \
//! 	# tweak the blocktime, if you want.
//! 	--consensus manual-seal-1000
//! ```
//!
//! This will launch your chain with no initial state, yay! Try reading a few keys from the state
//! now:
//!
//! ```text
//! wscat -c 127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"state_getStorage", "params": ["76616c7565"] }'
//! ```
//!
//! Can you guess what is this?
//!
//! ```text
//! wscat -c 127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"state_getStorage", "params": ["3a636f6465"] }'
//! ```
//!
//! If you want to try submitting a transaction, you can use the following:
//!
//! ```text
//! wscat -c ws://127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"author_submitExtrinsic", "params": [""]}'
//! ```
//!
//! You can use the `encode_examples` below to get an encoded extrinsic.
//!
//! ## Building a Chain Spec
//!
//! First, install the right version of the chain-spec builder:
//! ```text
//! cargo install staging-chain-spec-builder
//! ```
//!
//! ### Understanding Chain Specs
//!
//! Let's quickly explore how the chian-spec builder works:
//!
//! * The chain spec is a JSON file describing what the the initial specification of the chain is.
//! * It is an important feature of the omni-node-driven future of Polkadot.
//! * It contains many important pieces of information, but notable the genesis state, which we call
//!   `GenesisConfig`.
//!
//! The main flow of the chain-spec is as follows:
//!
//! * It is passed into the runtime, as a JSON encoded object, via the `build_state` API, and the
//!   runtime is responsible to build whatever is needed for it.
//!
//! The runtime will in itself expose instances of `GenesisConfig`, named `presets`. These are
//! various instances of the `GenesisConfig` that the runtime deems useful, for example one for
//! production, and one for testing.
//!
//! The runtime returns these presets via `get_preset` and `get_preset_names` APIs.
//!
//! Finally, the `chain-spec-builder` is a utility that communicates with the runtime, and creates a
//! chain-spec for you based on any of the presets, or the `Default` impl of `GenesisConfig` if none
//! is provided.
//!
//! You are always welcome to edit the final JSON file with hand as well, of course.
//!
//! The following diagram demonstrates all of this:
#![doc = simple_mermaid::mermaid!("../spec.mmd" left)]
//!
//! ### Launching a Chain With Chain Spec
//! ```text
//! chain-spec-builder create \
//! 	--chain-name pba-frameless \
//! 	-r ./target/release/wbuild/runtime/runtime.compact.compressed.wasm \
//! 	default
//! ```
//!
//! And inspect the value. See how you can find a `value: 42` in there. This is the genesis part of
//! your chain-spec, generated by the runtime, in the default *preset*.
//!
//! You can then run this with:
//! ```text
//! RUST_LOG=frameless=trace \
//! 	pba-omni-node \
//! 	--chain chain_spec.json \
//! 	--tmp
//! ```
//!
//! Notice how you can change the `value: 42` part and re-launch the chain. Of course, the initial
//! value of [`VALUE_KEY`] is the new value that you assign in the JSON file.
//!
//! ## Generating Presets
//!
//! The runtime can expose a fixed list of "presets" as the genesis config. You can see these in
//! [`Runtime::do_get_preset`] and [`Runtime::do_preset_names`]. For example:
//! ```text
//! chain-spec-builder create \
//! 	--chain-name pba-frameless \
//! 	-r ./target/release/wbuild/runtime/runtime.compact.compressed.wasm named-preset \
//! 	special-preset-1
//! ```
//!
//! Generates a chain-spec with [`VALUE_KEY`] set to `42 * 2`, because our template defines it as
//! so.
//!
//! You can read more about chain-specs [here](https://paritytech.github.io/polkadot-sdk/master/sc_chain_spec/index.html#).
//! Also: <https://github.com/paritytech/polkadot-sdk/pull/4678>
//!
//! ## More RPC Play
//!
//! Try the following RPC methods on both your node, and a live network, like Westend
//! (`wss://polkadot-rpc.dwellir.com`)
//!
//! * `state_getRuntimeVersion`
//! * `state_getMetadata`
//! * `system_version`
//! * `chain_getBlockHash`
//! * `chain_getBlock`

// because we run the docs with `--document-private-items`
#![allow(rustdoc::private_intra_doc_links)]
// The following 3 lines are related to your WASM build. Don't change.
#![cfg_attr(not(feature = "std"), no_std)]

use scale_info::TypeInfo;
// imports the `substrate`'s WASM-compatible standard library. This should give you all standard
// items like `vec!`. Do NOT bring in `std` from Rust, as this will not work in WASM.
use sp_std::prelude::*;

// The log target we will use this crate.
const LOG_TARGET: &'static str = "frameless";

use log::info;
use parity_scale_codec::{Compact, Decode, Encode};
use sp_api::impl_runtime_apis;
use sp_core::{hexdisplay::HexDisplay, OpaqueMetadata, H256};
use sp_runtime::{
	create_runtime_str, generic,
	traits::{BlakeTwo256, Block as BlockT, Hash},
	transaction_validity::{TransactionSource, TransactionValidity, ValidTransactionBuilder},
	ApplyExtrinsicResult, ExtrinsicInclusionMode,
};
use sp_version::RuntimeVersion;

/// The key to which [`Call::SetValue`] will write the value.
///
/// Hex: 0x76616c7565
const VALUE_KEY: &[u8] = b"value";
/// Temporary key used to store the header. This should always be clear at the end of the block.
///
/// Hex: 0x686561646572
const HEADER_KEY: &[u8] = b"header";
/// Key used to store all extrinsics in a block.
///
/// Should always remain in state at the end of the block, and be flushed at the beginning of the
/// next block.
const EXTRINSICS_KEY: &[u8] = b"extrinsics";

/// The block number type. You should not change this.
type BlockNumber = u32;

/// Signature type. We use `sr25519` crypto. You should not change this.
type Signature = sp_core::sr25519::Signature;
/// Account id type is the public key. We use `sr25519` crypto.
///
/// be aware of using the right crypto type when using `sp_keyring` and similar crates.
type AccountId = sp_core::sr25519::Public;

#[derive(
	Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
enum Call {
	SetValue { value: u32 },
	UpgradeCode { code: Vec<u8> },
}

#[derive(TypeInfo, Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
struct SignedExtrinsic {
	function: Call,
	signature: Option<(AccountId, Signature)>,
}

// A custom decode impl for your extrinsic type. It is important that this is double-encode and
// double decoded. Can you think why?
impl Decode for SignedExtrinsic {
	fn decode<I: parity_scale_codec::Input>(
		input: &mut I,
	) -> Result<Self, parity_scale_codec::Error> {
		// ignore the first byte, such that we can also decode as Vec<u8>.
		let _ = Compact::<u32>::decode(input)?;

		let function: Call = Decode::decode(input)?;
		let signature: Option<(AccountId, Signature)> = Decode::decode(input)?;

		Ok(SignedExtrinsic { function, signature })
	}
}

impl Encode for SignedExtrinsic {
	fn encode(&self) -> Vec<u8> {
		let mut result = Vec::new();
		let function_enc = self.function.encode();
		let signature_enc = self.signature.encode();
		let len = Compact::<u32>::from((function_enc.len() + signature_enc.len()) as u32);

		result.extend_from_slice(len.encode().as_ref());
		result.extend_from_slice(&function_enc);
		result.extend_from_slice(&signature_enc);
		result
	}
}

impl sp_runtime::traits::Extrinsic for SignedExtrinsic {
	type Call = Call;
	type SignaturePayload = ();
	fn new(call: Self::Call, _signed_data: Option<Self::SignaturePayload>) -> Option<Self> {
		Some(SignedExtrinsic { function: call, signature: None })
	}
	fn is_signed(&self) -> Option<bool> {
		Some(self.signature.is_some())
	}
}

/// The header type of the runtime.
type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// The block type of the runtime.
type Block = generic::Block<Header, SignedExtrinsic>;

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

/// The main struct in this module. In frame this comes from `construct_runtime!` macro.
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct Runtime;

// This impl block contains just some utilities that we have provided for you. You are free to use
// or ignore them.
#[allow(unused)]
impl Runtime {
	/// Print the entire state as a `trace` log.
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

	/// Get the state value at `key`, expected to decode into `T`.
	fn get_state<T: Decode>(key: &[u8]) -> Option<T> {
		sp_io::storage::get(key).and_then(|d| T::decode(&mut &*d).ok())
	}

	/// Mutate the value under `key`, expected to be of type `T` using `update`.
	///
	/// `update` contains `Some(T)` if a value exists under `T`, `None` otherwise
	fn mutate_state<T: Decode + Encode + Default>(key: &[u8], update: impl FnOnce(&mut T)) {
		let mut value = Self::get_state(key).unwrap_or_default();
		update(&mut value);
		sp_io::storage::set(key, &value.encode());
	}
}

/// A struct that defines the the genesis configuration of the runtime.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct RuntimeGenesis {
	pub(crate) value: u32,
}

// This impl block contains the core runtime api implementations. It contains good starting points
// denoted as a `FIXME`.
impl Runtime {
	pub(crate) fn do_initialize_block(
		header: &<Block as BlockT>::Header,
	) -> ExtrinsicInclusionMode {
		sp_io::storage::set(&HEADER_KEY, &header.encode());
		sp_io::storage::clear(&EXTRINSICS_KEY);
		ExtrinsicInclusionMode::AllExtrinsics
	}

	pub(crate) fn do_finalize_block() -> <Block as BlockT>::Header {
		// fetch the header that was given to us at the beginning of the block.
		let mut header = Self::get_state::<<Block as BlockT>::Header>(HEADER_KEY)
			.expect("We initialized with header, it never got mutated, qed");

		// and make sure to _remove_ it.
		sp_io::storage::clear(&HEADER_KEY);

		// This print is only for logging and debugging. Remove it.
		Runtime::print_state();

		let raw_state_root = &sp_io::storage::root(VERSION.state_version())[..];
		let state_root = sp_core::H256::decode(&mut &raw_state_root[..]).unwrap();

		let extrinsics = Self::get_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY).unwrap_or_default();
		let extrinsics_root = BlakeTwo256::ordered_trie_root(extrinsics, Default::default());

		header.extrinsics_root = extrinsics_root;
		header.state_root = state_root;
		header
	}

	/// Apply a single extrinsic.
	///
	/// In our template, we call into this from both block authoring, and block import.
	pub(crate) fn do_apply_extrinsic(ext: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
		let dispatch_outcome = match ext.clone().function {
			_ => Ok(()),
		};

		log::debug!(target: LOG_TARGET, "dispatched {:?}, outcome = {:?}", ext, dispatch_outcome);

		// note the extrinsic
		Self::mutate_state::<Vec<Vec<u8>>>(EXTRINSICS_KEY, |current| {
			current.push(ext.encode());
		});

		Ok(dispatch_outcome)
	}

	/// Your code path to execute a block that has been previously authored.
	pub(crate) fn do_execute_block(block: Block) {
		// clear any previous extrinsics. data.
		// NOTE: Look into FRAME, namely the system and executive crates and see if this is any
		// different in FRAME?
		sp_io::storage::clear(&EXTRINSICS_KEY);

		for extrinsic in block.clone().extrinsics {
			// the panic here is expected -- remember that we are in the block import code path now.
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
		let extrinsics_root = BlakeTwo256::ordered_trie_root(extrinsics, Default::default());
		assert_eq!(block.header.extrinsics_root, extrinsics_root);

		info!(target: LOG_TARGET, "Finishing block import.");
	}

	pub(crate) fn do_build_state(runtime_genesis: RuntimeGenesis) -> sp_genesis_builder::Result {
		sp_io::storage::set(&VALUE_KEY, &runtime_genesis.value.encode());
		Ok(())
	}

	pub(crate) fn do_get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
		match id {
			Some(preset_id) =>
				if preset_id.as_ref() == "special-preset-1".as_bytes() {
					Some(
						serde_json::to_string(&RuntimeGenesis { value: 42 * 2 })
							.unwrap()
							.as_bytes()
							.to_vec(),
					)
				} else {
					None
				},
			// none indicates the default preset.
			None => Some(
				serde_json::to_string(&RuntimeGenesis { value: 42 })
					.unwrap()
					.as_bytes()
					.to_vec(),
			),
		}
	}

	pub(crate) fn do_preset_names() -> Vec<sp_genesis_builder::PresetId> {
		vec!["special-preset-1".into()]
	}

	pub(crate) fn do_validate_transaction(
		_source: TransactionSource,
		tx: <Block as BlockT>::Extrinsic,
		block_hash: <Block as BlockT>::Hash,
	) -> TransactionValidity {
		log::debug!(target: LOG_TARGET, "Entering validate_transaction. tx: {:?}", tx);
		// Nothing to do for now, all is valid. We just provide a random tag as it is required.
		Ok(ValidTransactionBuilder::default()
			.and_provides((block_hash, tx).encode())
			.build()
			.unwrap())
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
			// Be aware: In your local tests, we assume `do_execute_block` is equal to
			// `execute_block`.
			Self::do_execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
			info!(
				target: LOG_TARGET,
				"Entering initialize_block. header: {:?} / version: {:?}", header, VERSION.spec_version
			);
			// Be aware: In your local tests, we assume `do_initialize_block` is equal to
			// `initialize_block`.
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
			_source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			_block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			log::debug!(target: LOG_TARGET,"Entering validate_transaction. tx: {:?}", tx);
			Self::do_validate_transaction(_source, tx, _block_hash)
		}
	}

	impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
		fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
			let runtime_genesis: RuntimeGenesis = serde_json::from_slice(&config)
				.map_err(|e| sp_runtime::format_runtime_string!("Invalid JSON blob: {}", e))?;
			info!(target: LOG_TARGET, "Entering build_state: {:?}", runtime_genesis);
			Self::do_build_state(runtime_genesis)
		}

		fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
			info!(target: LOG_TARGET, "Entering get_preset: {:?}", id);
			Self::do_get_preset(id)
		}

		fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
			info!(target: LOG_TARGET, "Entering preset_names");
			Self::do_preset_names()
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			// This is a FRAME-Less runtime, we have no clue what its metadata is 🤷
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
	use parity_scale_codec::Encode;
	use sp_core::hexdisplay::HexDisplay;
	use sp_io::TestExternalities;
	use sp_runtime::traits::Extrinsic as _;

	/// Return the list of extrinsics that are noted in the `EXTRINSICS_KEY`.
	fn noted_extrinsics() -> Vec<Vec<u8>> {
		sp_io::storage::get(EXTRINSICS_KEY)
			.and_then(|bytes| <Vec<Vec<u8>> as Decode>::decode(&mut &*bytes).ok())
			.unwrap_or_default()
	}

	/// Author a block with the given extrinsics, using the given state. Updates the state on
	/// the fly (for potential further inspection), and return the authored block.
	fn author_block(exts: Vec<SignedExtrinsic>, state: &mut TestExternalities) -> Block {
		let header = Header {
			digest: Default::default(),
			extrinsics_root: Default::default(),
			parent_hash: Default::default(),
			number: 0, // We don't care about block number here, just set it to 0.
			state_root: Default::default(),
		};

		state.execute_with(|| {
			Runtime::do_initialize_block(&header);
			drop(header);

			let mut extrinsics = vec![];
			for ext in exts {
				match Runtime::do_apply_extrinsic(ext.clone()) {
					Ok(_) => extrinsics.push(ext),
					Err(_) => (),
				}
			}

			let header = Runtime::do_finalize_block();

			assert!(
				sp_io::storage::get(HEADER_KEY).is_none(),
				"header must have been cleared from storage"
			);

			let onchain_noted_extrinsics = noted_extrinsics();
			assert_eq!(
				onchain_noted_extrinsics,
				extrinsics.iter().map(|e| e.encode()).collect::<Vec<_>>(),
				"incorrect extrinsics_key recorded in state"
			);

			let expected_state_root = {
				let raw_state_root = &sp_io::storage::root(Default::default())[..];
				H256::decode(&mut &raw_state_root[..]).unwrap()
			};
			let expected_extrinsics_root =
				BlakeTwo256::ordered_trie_root(onchain_noted_extrinsics, Default::default());

			assert_eq!(
				header.state_root, expected_state_root,
				"block finalization should set correct state root in header"
			);
			assert_eq!(
				header.extrinsics_root, expected_extrinsics_root,
				"block finalization should set correct extrinsics root in header"
			);

			Block { extrinsics, header }
		})
	}

	/// Import the given block
	fn import_block(block: Block, state: &mut TestExternalities) {
		state.execute_with(|| {
			// This should internally check state/extrinsics root. If it does not panic, then we
			// are gucci.
			Runtime::do_execute_block(block.clone());

			// double check the extrinsic and state root. `do_execute_block` must have already done
			// this, but better safe than sorry.
			assert_eq!(
				block.header.state_root,
				H256::decode(&mut &sp_io::storage::root(Default::default())[..][..]).unwrap(),
				"incorrect state root in authored block after importing"
			);
			assert_eq!(
				block.header.extrinsics_root,
				BlakeTwo256::ordered_trie_root(
					block.extrinsics.into_iter().map(|e| e.encode()).collect::<Vec<_>>(),
					Default::default()
				),
				"incorrect extrinsics root in authored block",
			);
		});
	}

	#[test]
	fn does_it_print() {
		// runt this with `cargo test does_it_print -- --nocapture`. Or if the test fails it will
		// also print.
		//
		// Note that WASM cannot print using `println!`! This can only be used in `cargo
		// test`, or wrapped in `sp_std::if_std! {}`, and then in native execution it will be seen.
		// In general, don't use this in your code. Use a proper logger, as described below.
		println!("Something");
	}

	#[test]
	fn does_it_log() {
		// run this with `RUST_LOG=frameless=trace cargo test -p runtime does_it_log``
		sp_tracing::try_init_simple();
		log::info!(target: LOG_TARGET, "Something");
	}

	#[test]
	fn host_function_call_works() {
		// this is just to demonstrate to you that you should always wrap any code containing host
		// functions in `TestExternalities`.
		TestExternalities::new_empty().execute_with(|| {
			println!("it works! {:?}", sp_io::storage::get(&VALUE_KEY));
		})
	}

	#[test]
	fn encode_examples() {
		// demonstrate some basic encodings. Example usage:
		//
		// ```
		// wscat -c ws://127.0.0.1:9944 -x '{"jsonrpc":"2.0", "id":1, "method":"author_submitExtrinsic", "params": ["0x123"]}'
		// ```
		let call = Call::SetValue { value: 1234 };
		let unsigned_ext = SignedExtrinsic::new(call, None).unwrap();

		println!(
			"unsigned = {:?}, encoded {:?}",
			unsigned_ext,
			HexDisplay::from(&unsigned_ext.encode()),
		);
	}

	#[test]
	fn import_and_author_equal() {
		// a few dummy extrinsics. The last one won't even pass predispatch, so it won't be
		// noted.
		let ext1 = SignedExtrinsic::new(Call::SetValue { value: 77 }, None).unwrap();
		let ext2 = SignedExtrinsic::new(Call::SetValue { value: 78 }, None).unwrap();
		let ext3 = SignedExtrinsic::new(Call::SetValue { value: 79 }, None).unwrap();

		let mut authoring_state = TestExternalities::new_empty();

		let block = author_block(vec![ext1, ext2, ext3], &mut authoring_state);
		authoring_state.execute_with(|| assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), Some(79)));

		let mut import_state = TestExternalities::new_empty();
		import_block(block, &mut import_state);
		import_state.execute_with(|| assert_eq!(Runtime::get_state::<u32>(VALUE_KEY), Some(79)));
	}

	#[test]
	fn double_encoding_works() {
		// the purpose of the double encoding is as follows: the same encoded extrinsic can be
		// decoded both as the concrete extrinsic type of the runtime, and the opaque extrinsic
		let ext = SignedExtrinsic::new(Call::SetValue { value: 77 }, None).unwrap();
		let encoded = ext.encode();

		let _decoded_as_ext = <SignedExtrinsic as Decode>::decode(&mut &*encoded).unwrap();
		let _decoded_as_opaque =
			<sp_runtime::OpaqueExtrinsic as Decode>::decode(&mut &*encoded).unwrap();
	}
}
