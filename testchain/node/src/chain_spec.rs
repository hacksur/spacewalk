use spacewalk_runtime::{AssetId, DiaOracleModuleConfig};
use std::{convert::TryFrom, str::FromStr};

use frame_support::BoundedVec;
use hex_literal::hex;
use sc_service::ChainType;
use sp_arithmetic::{FixedPointNumber, FixedU128};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use primitives::{oracle::Key, CurrencyId, VaultCurrencyPair};
use serde_json::{map::Map, Value};
use spacewalk_runtime::{
	AccountId, AuraConfig, BalancesConfig, FeeConfig, FieldLength, GenesisConfig, GrandpaConfig,
	IssueConfig, NominationConfig, OracleConfig, Organization, RedeemConfig, ReplaceConfig,
	SecurityConfig, Signature, StatusCode, StellarRelayConfig, SudoConfig, SystemConfig,
	TokensConfig, Validator, VaultRegistryConfig, DAYS, WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

// For mainnet USDC issued by centre.io
// const WRAPPED_CURRENCY_ID: CurrencyId = CurrencyId::AlphaNum4 {
// 	code: *b"USDC",
// 	issuer: [
// 		59, 153, 17, 56, 14, 254, 152, 139, 160, 168, 144, 14, 177, 207, 228, 79, 54, 111, 125,
// 		190, 148, 107, 237, 7, 114, 64, 247, 246, 36, 223, 21, 197,
// 	],
// };
// For Testnet USDC issued by
const WRAPPED_CURRENCY_ID: CurrencyId = CurrencyId::AlphaNum4(
	*b"USDC",
	[
		20, 209, 150, 49, 176, 55, 23, 217, 171, 154, 54, 110, 16, 50, 30, 226, 102, 231, 46, 199,
		108, 171, 97, 144, 240, 161, 51, 109, 72, 34, 159, 139,
	],
);

const MXN_CURRENCY_ID: CurrencyId = CurrencyId::AlphaNum4(
	*b"MXN\0",
	[
		20, 209, 150, 49, 176, 55, 23, 217, 171, 154, 54, 110, 16, 50, 30, 226, 102, 231, 46, 199,
		108, 171, 97, 144, 240, 161, 51, 109, 72, 34, 159, 139,
	],
);

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

fn get_account_id_from_string(account_id: &str) -> AccountId {
	AccountId::from_str(account_id).expect("account id is not valid")
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn get_properties() -> Map<String, Value> {
	let mut properties = Map::new();
	properties.insert("ss58Format".into(), spacewalk_runtime::SS58Prefix::get().into());
	properties
}

pub fn local_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"spacewalk",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![],
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
				vec![get_account_id_from_seed::<sr25519::Public>("Bob")],
				false,
			)
		},
		vec![],
		None,
		None,
		None,
		Some(get_properties()),
		None,
	)
}

pub fn beta_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"spacewalk",
		"beta_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				get_account_id_from_string("5HeVGqvfpabwFqzV1DhiQmjaLQiFcTSmq2sH6f7atsXkgvtt"),
				vec![
					(
						// 5DJ3wbdicFSFFudXndYBuvZKjucTsyxtJX5WPzQM8HysSkFY
						hex!["366a092a27b4b28199a588b0155a2c9f3f0513d92481de4ee2138273926fa91c"]
							.unchecked_into(),
						hex!["dce82040dc0a90843897aee1cc1a96c205fe7c1165b8f46635c2547ed15a3013"]
							.unchecked_into(),
					),
					(
						// 5HW7ApFamN6ovtDkFyj67tRLRhp8B2kVNjureRUWWYhkTg9j
						hex!["f08cc7cf45f88e6dbe312a63f6ce639061834b4208415b235f77a67b51435f63"]
							.unchecked_into(),
						hex!["5b4651cf045ddf55f0df7bfbb9bb4c45bbeb3c536c6ce4a98275781b8f0f0754"]
							.unchecked_into(),
					),
					(
						// 5FNbq8zGPZtinsfgyD4w2G3BMh75H3r2Qg3uKudTZkJtRru6
						hex!["925ad4bdf35945bea91baeb5419a7ffa07002c6a85ba334adfa7cb5b05623c1b"]
							.unchecked_into(),
						hex!["8de3db7b51864804d2dd5c5905d571aa34d7161537d5a0045755b72d1ac2062e"]
							.unchecked_into(),
					),
				],
				vec![
					// root key
					get_account_id_from_string("5HeVGqvfpabwFqzV1DhiQmjaLQiFcTSmq2sH6f7atsXkgvtt"),
					// faucet
					get_account_id_from_string("5FHy3cvyToZ4ConPXhi43rycAcGYw2R2a8cCjfVMfyuS1Ywg"),
					// vaults
					get_account_id_from_string("5F7Q9FqnGwJmjLtsFGymHZXPEx2dWRVE7NW4Sw2jzEhUB5WQ"),
					get_account_id_from_string("5CJncqjWDkYv4P6nccZHGh8JVoEBXvharMqVpkpJedoYNu4A"),
					get_account_id_from_string("5GpnEWKTWv7xiQtDFi9Rku7DrvgHj4oqMDev4qBQhfwQE8nx"),
					get_account_id_from_string("5DttG269R1NTBDWcghYxa9NmV2wHxXpTe4U8pu4jK3LCE9zi"),
					// relayers
					get_account_id_from_string("5DNzULM1UJXDM7NUgDL4i8Hrhe9e3vZkB3ByM1eEXMGAs4Bv"),
					get_account_id_from_string("5GEXRnnv8Qz9rEwMs4TfvHme48HQvVTEDHJECCvKPzFB4pFZ"),
					// oracles
					get_account_id_from_string("5H8zjSWfzMn86d1meeNrZJDj3QZSvRjKxpTfuVaZ46QJZ4qs"),
					get_account_id_from_string("5FPBT2BVVaLveuvznZ9A1TUtDcbxK5yvvGcMTJxgFmhcWGwj"),
				],
				vec![get_account_id_from_seed::<sr25519::Public>("Bob")],
				false,
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(get_properties()),
		None,
	)
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"spacewalk",
		"dev_testnet",
		ChainType::Development,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![authority_keys_from_seed("Alice")],
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
				vec![get_account_id_from_seed::<sr25519::Public>("Bob")],
				false,
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(get_properties()),
		None,
	)
}

fn default_pair(currency_id: CurrencyId) -> VaultCurrencyPair<CurrencyId> {
	VaultCurrencyPair { collateral: currency_id, wrapped: WRAPPED_CURRENCY_ID }
}

// Used to create bounded vecs for genesis config
// Does not return a result but panics because the genesis config is hardcoded
fn create_bounded_vec(input: &str) -> BoundedVec<u8, FieldLength> {
	let bounded_vec =
		BoundedVec::try_from(input.as_bytes().to_vec()).expect("Failed to create bounded vec");

	bounded_vec
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
	authorized_oracles: Vec<AccountId>,
	start_shutdown: bool,
) -> GenesisConfig {
	// Testnet organization
	let organization_testnet_sdf = Organization { name: create_bounded_vec("sdftest"), id: 1u128 };
	// Testnet validators
	let validators = vec![
		Validator {
			name: create_bounded_vec("$sdftest1"),
			public_key: create_bounded_vec(
				"GDKXE2OZMJIPOSLNA6N6F2BVCI3O777I2OOC4BV7VOYUEHYX7RTRYA7Y",
			),
			organization_id: organization_testnet_sdf.id,
		},
		Validator {
			name: create_bounded_vec("$sdftest2"),
			public_key: create_bounded_vec(
				"GCUCJTIYXSOXKBSNFGNFWW5MUQ54HKRPGJUTQFJ5RQXZXNOLNXYDHRAP",
			),
			organization_id: organization_testnet_sdf.id,
		},
		Validator {
			name: create_bounded_vec("$sdftest3"),
			public_key: create_bounded_vec(
				"GC2V2EFSXN6SQTWVYA5EPJPBWWIMSD2XQNKUOHGEKB535AQE2I6IXV2Z",
			),
			organization_id: organization_testnet_sdf.id,
		},
	];
	let organizations = vec![organization_testnet_sdf];
	GenesisConfig {
		system: SystemConfig {
			code: WASM_BINARY.expect("WASM binary was not build, please build it!").to_vec(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		tokens: TokensConfig {
			// Configure the initial token supply for the native currency and USDC asset
			balances: endowed_accounts
				.iter()
				.flat_map(|k| {
					vec![
						(k.clone(), CurrencyId::XCM(0), 1 << 60),
						(k.clone(), CurrencyId::XCM(1), 1 << 60),
						(k.clone(), CurrencyId::Native, 1 << 60),
					]
				})
				.collect(),
		},
		issue: IssueConfig {
			issue_period: DAYS,
			issue_minimum_transfer_amount: 100_00000, // This is 100 Stellar stroops
			limit_volume_amount: None,
			limit_volume_currency_id: CurrencyId::XCM(0),
			current_volume_amount: 0u128,
			interval_length: (60u32 * 60 * 24),
			last_interval_index: 0u32,
		},
		redeem: RedeemConfig {
			redeem_period: DAYS,
			redeem_minimum_transfer_amount: 100_00000, // This is 100 Stellar stroops
			limit_volume_amount: None,
			limit_volume_currency_id: CurrencyId::XCM(0),
			current_volume_amount: 0u128,
			interval_length: (60u32 * 60 * 24),
			last_interval_index: 0u32,
		},
		replace: ReplaceConfig {
			replace_period: DAYS,
			replace_minimum_transfer_amount: 100_00000, // This is 100 Stellar stroops
		},
		security: SecurityConfig {
			initial_status: if start_shutdown { StatusCode::Shutdown } else { StatusCode::Error },
		},
		stellar_relay: StellarRelayConfig {
			old_validators: vec![],
			old_organizations: vec![],
			validators,
			organizations,
			enactment_block_height: 0,
			phantom: Default::default(),
		},
		oracle: OracleConfig {
			max_delay: u32::MAX,
			oracle_keys: vec![
				// Changing these items means that the integration tests also have to change
				// because the integration tests insert dummy values for these into the oracle
				Key::ExchangeRate(CurrencyId::XCM(0)),
				Key::ExchangeRate(WRAPPED_CURRENCY_ID),
				Key::ExchangeRate(MXN_CURRENCY_ID),
			],
		},
		vault_registry: VaultRegistryConfig {
			minimum_collateral_vault: vec![
				(CurrencyId::XCM(0), 0),
				(CurrencyId::XCM(1), 0),
				(CurrencyId::StellarNative, 0),
			],
			punishment_delay: DAYS,
			secure_collateral_threshold: vec![
				(
					default_pair(CurrencyId::XCM(0)),
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
				(
					default_pair(CurrencyId::XCM(1)),
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
				(
					VaultCurrencyPair { collateral: CurrencyId::XCM(0), wrapped: MXN_CURRENCY_ID },
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
				(
					VaultCurrencyPair {
						collateral: CurrencyId::XCM(0),
						wrapped: CurrencyId::StellarNative,
					},
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
			],
			/* 150% */
			premium_redeem_threshold: vec![
				(
					default_pair(CurrencyId::XCM(0)),
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
				(
					default_pair(CurrencyId::XCM(1)),
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
				(
					VaultCurrencyPair {
						collateral: CurrencyId::StellarNative,
						wrapped: MXN_CURRENCY_ID,
					},
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
				(
					VaultCurrencyPair { collateral: CurrencyId::XCM(0), wrapped: MXN_CURRENCY_ID },
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
				(
					VaultCurrencyPair {
						collateral: CurrencyId::XCM(0),
						wrapped: CurrencyId::StellarNative,
					},
					FixedU128::checked_from_rational(140, 100).unwrap(),
				),
			],
			/* 135% */
			liquidation_collateral_threshold: vec![
				(
					default_pair(CurrencyId::XCM(0)),
					FixedU128::checked_from_rational(120, 100).unwrap(),
				),
				(
					default_pair(CurrencyId::XCM(1)),
					FixedU128::checked_from_rational(120, 100).unwrap(),
				),
				(
					VaultCurrencyPair { collateral: CurrencyId::XCM(0), wrapped: MXN_CURRENCY_ID },
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
				(
					VaultCurrencyPair {
						collateral: CurrencyId::XCM(0),
						wrapped: CurrencyId::StellarNative,
					},
					FixedU128::checked_from_rational(160, 100).unwrap(),
				),
			],
			/* 110% */
			system_collateral_ceiling: vec![
				(default_pair(CurrencyId::XCM(0)), 60_000 * 10u128.pow(12)),
				(default_pair(CurrencyId::XCM(1)), 60_000 * 10u128.pow(12)),
				(
					VaultCurrencyPair { collateral: CurrencyId::XCM(0), wrapped: MXN_CURRENCY_ID },
					60_000 * 10u128.pow(12),
				),
				(
					VaultCurrencyPair {
						collateral: CurrencyId::XCM(0),
						wrapped: CurrencyId::StellarNative,
					},
					60_000 * 10u128.pow(12),
				),
			],
		},
		fee: FeeConfig {
			issue_fee: FixedU128::checked_from_rational(1, 1000).unwrap(), // 0.1%
			issue_griefing_collateral: FixedU128::checked_from_rational(5, 1000).unwrap(), // 0.5%
			redeem_fee: FixedU128::checked_from_rational(1, 1000).unwrap(), // 0.1%
			premium_redeem_fee: FixedU128::checked_from_rational(5, 100).unwrap(), // 5%
			punishment_fee: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
			replace_griefing_collateral: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
		},
		nomination: NominationConfig { is_nomination_enabled: false },
		dia_oracle_module: DiaOracleModuleConfig {
			authorized_accounts: authorized_oracles,
			supported_currencies: vec![
				AssetId::new(b"Kusama".to_vec(), b"KSM".to_vec()),
				AssetId::new(b"Polkadot".to_vec(), b"DOT".to_vec()),
				// The order of currencies in the FIAT symbols matter and USD should always be the
				// target currency ie the second one in the pair
				AssetId::new(b"FIAT".to_vec(), b"USD-USD".to_vec()),
				AssetId::new(b"FIAT".to_vec(), b"MXN-USD".to_vec()),
				AssetId::new(b"Stellar".to_vec(), b"XLM".to_vec()),
			],
			batching_api: b"http://localhost:8070/currencies".to_vec(),
			coin_infos_map: vec![],
		},
	}
}
