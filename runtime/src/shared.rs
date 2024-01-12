//! Shared types used in your solution, and our grading tool.

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{generic, traits::BlakeTwo256};
use sp_std::prelude::*;

/// The Balance type. You should not change this.
pub type Balance = u128;
/// The block number type. You should not change this.
pub type BlockNumber = u32;
/// Signature type. We use `sr25519` crypto. You should not change this.
pub type Signature = sp_core::sr25519::Signature;
/// Account id type is the public key. We use `sr25519` crypto.
///
/// be aware of using the right crypto type when using `sp_keyring` and similar crates.
pub type AccountId = sp_core::sr25519::Public;

/// The account id who's allowed to mint, and call `Sudo*` operations. This is the sr25519
/// representation of `Alice` in `sp-keyring`.
pub const SUDO: [u8; 32] =
	hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
/// The treasury account to which tips should be deposited.
pub const TREASURY: [u8; 32] =
	hex_literal::hex!["ff3593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
/// The key to which [`SystemCall::Set`] will write the value.
///
/// Hex: 0x76616c7565
pub const VALUE_KEY: &[u8] = b"value";
/// The key to which [`SystemCall::SudoSet`] will write the value.
///
/// Hex: 0x7375646f5f76616c7565
pub const SUDO_VALUE_KEY: &[u8] = b"sudo_value";
/// Temporary key used to store the header. This should always be clear at the end of the block.
///
/// Hex: 0x686561646572
pub const HEADER_KEY: &[u8] = b"header";
/// Key used to store all extrinsics in a block.
///
/// Should always remain in state at the end of the block, and be flushed at the beginning of the
/// next block.
pub const EXTRINSICS_KEY: &[u8] = b"extrinsics";

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum SystemCall {
	/// Do nothing.
	///
	/// This will only ensure that `data` is remarked as the block data, but nothing is changed in
	/// the state (other than potential side effects related to tipping and nonce).
	///
	/// ## Dispatch Errors
	///
	/// None. Should always work.
	Remark { data: sp_std::prelude::Vec<u8> },
	/// Set the value under [`VALUE_KEY`] to `value`. Can be dispatched by anyone.
	///
	/// ## Dispatch Errors
	///
	/// None. Should always work.
	Set { value: u32 },
	/// Set the value under [`SUDO_VALUE_KEY`] to `value`.
	///
	/// ## Dispatch Errors
	///
	/// [`sp_runtime::DispatchError::BadOrigin`] if the caller is not [`SUDO`].
	SudoSet { value: u32 },
	/// Upgrade the runtime to the given code. In a real world situation, this should be heavily
	/// permissioned.
	///
	/// This is only for you to play around with, and no test will use it.
	///
	/// ## Dispatch Errors
	///
	/// None. Should always work.
	Upgrade { code: sp_std::prelude::Vec<u8> },
}

/// This is the amount of **FREE** (see [`AccountBalance`] balance that is needed for any account to
/// exist. In other words, at NO POINT IN TIME an account's free balance should be less than this
/// amount.
pub const EXISTENTIAL_DEPOSIT: Balance = 10;

/// The specification of dispatchable calls in the currency module.
///
/// This module is expected to maintain a total issuance. This is a single value of type [`Balance`]
/// that should be the sum of **ALL** account balances that exists. No exceptions.
///
/// Refer to each variant for more information.
///
/// ## Storage Layout
///
/// * mapping [`AccountId`] to [`AccountBalance`] kept at `BalancesMap + encode(account)`.
/// * value of type [`Balance`] for total issuance kept at `TotalIssuance`.
///
/// ```
/// use sp_keyring::AccountKeyring;
/// use parity_scale_codec::Encode;
/// fn main() {
///     // storage key for alice.
///     let alice = AccountKeyring::Alice.public();
///     let key = [b"BalancesMap".as_ref(), alice.as_ref()].concat();
///
///     // notice that encoding of certain types are themselves.
///     assert_eq!(b"BalancesMap".as_ref(), b"BalancesMap".encode());
///     assert_eq!(alice.as_ref(), alice.encode());
/// }
/// ```
///
/// ## Existential Deposit
///
/// > Revisit this section once you read about staking and tipping as well.
///
/// Accounts that are stored in storage, as per mapping explained above, should at least have 10
/// units of "free" balance. You can think of this as an upfront deposit. In order for the runtime
/// to consider an account "worthy" of taking up space in the state, it must at least bring 10 units
/// of free balance to the system.
///
/// We define 3 states for an account:
///
/// * Created, exists: it means it has more than [`EXISTENTIAL_DEPOSIT`] units of FREE balance. So
///   long as enough free balance exists, the reserved balance is irrelevant.
/// * Destroyed: When an account has no free AND no reserved balance left, it is destroyed. This
///   means its associated item in the balances map is REMOVED.
/// * Invalid: Any other combination of free and reserved balance is invalid.
///
/// In all transactions:
///
/// - The sender of a transaction must exist prior to applying the transaction. The sender might
///   finish the transaction while still existing, or destroyed.
/// - Similarly, any other parties involved in the transaction must not finish the transaction in
///   the invalid state.
///
/// ## Apply Errors
///
/// Knowing the above, you must add a new check to your apply conditions (such as signature check),
/// that prior to being applied, all transactions must come from a sender with an "existing"
/// account.
///
/// (This probably forces you to update your tests to pre-fund a few accounts.)
///
/// ## Transaction Pool Validation Errors
///
/// Same as Apply errors. The existence of accounts must be checked at `validate_transaction`.
///
/// ## Dispatch Errors
///
/// Explained at each variant below.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum CurrencyCall {
	/// Mint `amount` of tokens to `dest`. This will increase the total issuance of the system.
	///
	/// If `dest` exists, its balance is increased. Else, it is created, if possible.
	///
	/// ## Dispatch Errors
	///
	/// * [`sp_runtime::DispatchError::BadOrigin`] if the sender is not [`SUDO`].
	/// * [`sp_runtime::DispatchError::Token`] if the `dest` ends up in an invalid state with
	///   respect to existential deposit.
	/// * [`sp_runtime::DispatchError::Arithmetic`] if any type of arithmetic operation overflows.
	Mint { dest: AccountId, amount: Balance },
	/// Transfer `amount` to `dest`.
	///
	/// The `sender` must exist prior to applying the transaction, but the `dest` might be created
	/// in the process. The sender might get destroyed as a consequence of dispatch.
	///
	/// ## Dispatch Errors
	///
	/// * [`sp_runtime::DispatchError::Token`] If either `sender` or `dest` end up in an invalid
	///   state with respect to existential deposit.
	/// * [`sp_runtime::DispatchError::Arithmetic`] if any type of arithmetic operation overflows.
	Transfer { dest: AccountId, amount: Balance },
	/// Alias for `Transfer { dest, amount: sender.free }`.
	TransferAll { dest: AccountId },
}

/// The specification of dispatchable calls in the currency module.
///
/// This module has no additional storage, and utilizes the existing [`AccountBalance`] stored under
/// `BalancesMap`.
///
/// ## Apply Errors
///
/// No additional apply errors.
///
/// ## Transaction Pool Validation Errors
///
/// No additional pool validation errors.
///
/// ## Dispatch Errors
///
/// Explained at each variant below.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum StakingCall {
	/// Bond `amount` form the sender, if they have enough free balance.
	///
	/// This results in `amount` being moved from their free balance to their reserved balance.
	///
	/// ## Dispatch Errors
	///
	/// * [`sp_runtime::DispatchError::BadOrigin`] if the sender does not exist.
	/// * [`sp_runtime::DispatchError::Token`] If `sender` ends up in an invalid state with respect
	///   to existential deposit.
	/// * [`sp_runtime::DispatchError::Arithmetic`] if any type of arithmetic operation overflows.
	Bond { amount: Balance },
}

/// The outer runtime call.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum RuntimeCall {
	Currency(CurrencyCall),
	Staking(StakingCall),
	System(SystemCall),
}

/// Extended, final runtime call.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub struct RuntimeCallExt {
	/// The callable operation.
	pub call: RuntimeCall,
	/// The nonce.
	pub nonce: u32,
	/// Optional tip.
	pub tip: Option<Balance>,
}

/// Final extrinsic type of the runtime.
///
/// Our tests will use the given types to interact with your runtime. Note that you can use any
/// other type on your runtime type, as long as you can convert it to/from these types.
#[docify::export]
pub type Extrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCallExt, Signature, ()>;

/// The header type of the runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// The block type of the runtime.
pub type Block = generic::Block<Header, Extrinsic>;

/// The account balance struct that we expect to find under `BalancesMap ++ account_id`.
///
/// The free balance of an account is the subset of the account balance that can be transferred
/// out of the account. As noted elsewhere, the free balance of ALL accounts at ALL TIMES mut be
/// equal or more than that of [`EXISTENTIAL_DEPOSIT`].
///
/// Conversely, the reserved part of an account is a subset that CANNOT be transferred out,
/// unless if explicitly unreserved.
#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, Default)]
pub struct AccountBalance {
	/// The free balance that they have. This can be transferred.
	free: Balance,
	/// The reserved balance that they have. This CANNOT be transferred.
	reserved: Balance,
	/// The nonce of the account. Increment every time an account successfully transacts.
	///
	/// Once an account is created, it should have a nonce of 0. By the end of the transaction,
	/// this value is increment to 1.
	nonce: u32,
}

impl AccountBalance {
	/// Create a new instance of `Self`.
	///
	/// This ensures that no instance of this type is created by mistake with less than
	/// `EXISTENTIAL_DEPOSIT`.
	pub fn new_from_free(free: Balance) -> Self {
		assert!(free >= EXISTENTIAL_DEPOSIT, "free balance must be at least EXISTENTIAL_DEPOSIT");
		Self { free, ..Default::default() }
	}
}
