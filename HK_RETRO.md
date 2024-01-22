

---

Don't tweak this line, or generally anything else that is std/no-std related

```
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
```

In general, we should have emphasized more about https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/substrate/index.html#wasm-build


---

It is unfortunate to see so many people fail some of the basic currency tests, but having implemented Staking or other features that were optional.

---

Use a formatter! it is good. Makes life easier.

---

Logging etc.

---

```
if ext.function.tip.is_some() {
	let tip_amount = ext.function.tip.unwrap();
```

---

```
let signer_balance = Runtime::get_state::<AccountBalance>(&Self::get_storage_key(&signer));
match signer_balance {
	Some(_) => (),
	None => {return Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner))},
};

match Self::apply_predispatch(&ext, signer) {
	Ok(_) => (),
	Err(e) => {return Err(e)},
};
```

```
let signer_balance = Runtime::get_state::<AccountBalance>(&Self::get_storage_key(&signer))
	.ok_or(TransactionValidityError::Invalid(InvalidTransaction::BadSigner))?;

```

---

Safe math. We didn't check it a lot in this assignment, but next week you should know better.

---

`is_some` and `is_none`

```
let account_balance = Runtime::get_state::<AccountBalance>(&balance_key(account)); //.unwrap_or_default();
if account_balance.is_none() {
	return false;
}
true
```

---

