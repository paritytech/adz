use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachain_runtime::{AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				1000.into(),
			)
		},
		vec![],
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn adz_westend() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());

	ChainSpec::from_genesis(
		// Name
		"Adz Westend",
		// ID
		"adz_westend",
		ChainType::Live,
		move || {
			testnet_genesis(
				//5H8UXihvYfKtvBJpJS4cfKsAFMhz4UWZ6MnXE5H2bWtT9u8Y
				hex!["e00d4985a0ef8c7821a50fcdeb2d5cd4f0874c11bc00532157fcac6400b49073"].into(),
				vec![
					//5FW42aqrsnDPmTsrjVC1BfA8RLurURE8CmsSSeqLsTh1PGah
					hex!["9809bca9bafffacc2fccb3884afb6e3d2b96187d03a39fd8807e34dc9bd4d277"]
						.unchecked_into(),
					//5EZ4MwVEaebH1GvvEWjVEp3n7CwYLSNewMxF5ePirqY255g8
					hex!["6e185cf68b2fb6947b8645a5b73081e5ca03ce2ce1bc31ac2d2cc352cfc52b6c"]
						.unchecked_into(),
					//5GsyP8HwkvKYoyYbPwjcR88KLKtobzHuV5cMB8qg23YtKBck
					hex!["d4fdbae660fc34a80927ef4d5e099bec2b7241ff7c8bde51edd65542cce33a7c"]
						.unchecked_into(), //5EqnP2zoBgJQRnragZ59oPid1xWebNWegzChm8MeKMyVRp6a
					hex!["7ad9bcde098a69b9f3c291e2d4d016e9f7d98c7614479244440f96f4f6b5be47"]
						.unchecked_into(),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Angelspit"),
					get_account_id_from_seed::<sr25519::Public>("Choking Victim"),
					get_account_id_from_seed::<sr25519::Public>("White Ward"),
					get_account_id_from_seed::<sr25519::Public>("Android Lust"),
					get_account_id_from_seed::<sr25519::Public>("Zhine"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "westend".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
	parachain_runtime::GenesisConfig {
		system: parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: parachain_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		sudo: parachain_runtime::SudoConfig { key: Some(root_key) },
		parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
		aura: parachain_runtime::AuraConfig { authorities: initial_authorities },
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
