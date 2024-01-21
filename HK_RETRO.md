

---

Don't tweak this line, or generally anything else that is std/no-std related

```
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
```

In general, we should have emphasized more about https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/substrate/index.html#wasm-build


---

It is unfortunate to see so many people fail some of the basic currency tests, but having implemented Staking or other features that were optional.
