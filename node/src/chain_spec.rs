use runtime::WASM_BINARY;
use sc_service::{ChainType, Properties};

pub type ChainSpec = sc_service::GenericChainSpec<()>;

fn props() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".to_string(), 0.into());
	properties.insert("tokenSymbol".to_string(), "TEST".into());
	properties
}

pub fn development_config() -> Result<ChainSpec, String> {
	let cs = ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_properties(props())
	.with_chain_type(ChainType::Development)
	.build();
	Ok(cs)
}
