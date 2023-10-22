// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use gos_runtime::{
	constants::currency::*, wasm_binary_unwrap, BabeConfig, BalancesConfig, Block, CouncilConfig,
	DemocracyConfig, ElectionsConfig, ImOnlineConfig, IndicesConfig, MaxNominations,
	NominationPoolsConfig, SessionConfig, SessionKeys, StakerStatus, StakingConfig,
	SudoConfig, SystemConfig, TechnicalCommitteeConfig,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{
	crypto::{Ss58Codec,UncheckedInto,AccountId32}, 
	sr25519, Pair, Public
};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use serde_json::json;
pub use gos_runtime::RuntimeGenesisConfig;
pub use node_primitives::{AccountId, Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

const SYMBOL_TEST : &str = "GOST";
const SYMBOL : &str = "GOS";
const DECIMALS : i32 = 12;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> RuntimeGenesisConfig {
	#[rustfmt::skip]
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
			array_bytes::hex_n_into_unchecked("9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"),
			// 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
			array_bytes::hex_n_into_unchecked("781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"),
			// 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
			array_bytes::hex2array_unchecked("9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332")
				.unchecked_into(),
			// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
			array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
				.unchecked_into(),
			// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
			array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
				.unchecked_into(),
			// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
			array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
				.unchecked_into(),
		),
		(
			// 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
			array_bytes::hex_n_into_unchecked("68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"),
			// 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
			array_bytes::hex_n_into_unchecked("c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"),
			// 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
			array_bytes::hex2array_unchecked("7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f")
				.unchecked_into(),
			// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
			array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
				.unchecked_into(),
			// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
			array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
				.unchecked_into(),
			// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
			array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
				.unchecked_into(),
		),
		(
			// 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
			array_bytes::hex_n_into_unchecked("547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"),
			// 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
			array_bytes::hex_n_into_unchecked("9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"),
			// 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
			array_bytes::hex2array_unchecked("5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440")
				.unchecked_into(),
			// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
			array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
				.unchecked_into(),
			// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
			array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
				.unchecked_into(),
			// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
			array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
				.unchecked_into(),
		),
		(
			// 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
			array_bytes::hex_n_into_unchecked("f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"),
			// 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
			array_bytes::hex_n_into_unchecked("66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"),
			// 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
			array_bytes::hex2array_unchecked("3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef")
				.unchecked_into(),
			// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
			array_bytes::hex2array_unchecked("00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378")
				.unchecked_into(),
			// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
			array_bytes::hex2array_unchecked("00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378")
				.unchecked_into(),
			// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
			array_bytes::hex2array_unchecked("00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378")
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = array_bytes::hex_n_into_unchecked(
		// 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
		"9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809",
	);

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	genesis_config(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	// configure GOS currency symbol for the native coin
	let symbol_value = json!(SYMBOL_TEST);
	let token_decimals = json!(DECIMALS);

	let mut gos_props : Properties = Properties::new();
	gos_props.insert("tokenSymbol".to_string(), symbol_value);
	gos_props.insert("tokenDecimals".to_string(), token_decimals);

	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"GOS Staging Testnet",
		"gos_staging_testnet",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		// Protocol ID
		Some("gost"),
		None,
		// Properties
		Some(gos_props),
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to generate stash, controller and session key for account address.
pub fn authority_keys_from_acct(
	sudo_str: &str,
	sudo_stash_str: &str,
	gran_str: &str
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	
	let sudo_account: AccountId = Ss58Codec::from_ss58check(sudo_str).unwrap();
	let sudo_stash: AccountId = Ss58Codec::from_ss58check(sudo_stash_str).unwrap();
	let grandpa_account: GrandpaId = Ss58Codec::from_string(gran_str).unwrap();
	let babe_account: BabeId = Ss58Codec::from_string(sudo_str).unwrap();
	let im_account: ImOnlineId = Ss58Codec::from_string(sudo_str).unwrap();
	let auth_discovery_account: AuthorityDiscoveryId = Ss58Codec::from_string(sudo_str).unwrap();
	(
		sudo_stash.clone(),
		sudo_account.clone(),
		grandpa_account,
		babe_account,
		im_account,
		auth_discovery_account,
	)
}
/// Helper function to decode ss58 to AccountId32
pub fn ss58_to_account_id(ss58 : &str) -> Result<AccountId32, &str> {
	let decoded = AccountId32::from_ss58check(ss58).map_err(|_| "Failed to decode SS58 address");
	decoded
}

/// Helper function to create RuntimeGenesisConfig for testing.
pub fn genesis_config(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> RuntimeGenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
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
		]
	});
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.0.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;

	RuntimeGenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec(), ..Default::default() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			epoch_config: Some(gos_runtime::BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: Default::default(),
		grandpa: Default::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),
		vesting: Default::default(),
		// assets: pallet_assets::GenesisConfig {
		// 	// This asset is used by the NIS pallet as counterpart currency.
		// 	assets: vec![(0, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1)],
		// 	..Default::default()
		// },
		// pool_assets: Default::default(),
		transaction_storage: Default::default(),
		transaction_payment: Default::default(),
		safe_mode: Default::default(),
		tx_pause: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: 1 * DOLLARS,
			..Default::default()
		},
		glutton: Default::default(),
	}
}

fn development_config_genesis() -> RuntimeGenesisConfig {
	genesis_config(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Development config (single validator Alice).
pub fn development_config() -> ChainSpec {
	
	// configure GOS currency symbol for the native coin
	let symbol_value = json!(SYMBOL);
	let token_decimals = json!(DECIMALS);

	let mut gos_props : Properties = Properties::new();
	gos_props.insert("tokenSymbol".to_string(), symbol_value);
	gos_props.insert("tokenDecimals".to_string(), token_decimals);

	ChainSpec::from_genesis(
		"Spectral Dev",
		"dev",
		ChainType::Development,
		development_config_genesis,
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("gos"),
		None,
		// Properties
		Some(gos_props),
		Default::default(),
	)
}

fn local_testnet_genesis() -> RuntimeGenesisConfig {
	genesis_config(
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob).
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		None,
		Default::default(),
	)
}

/// Genesis Testnet Configuration Settings
fn spectral_config_genesis() -> RuntimeGenesisConfig {
	
	// GOS SUDO, STASH, GRANDPA
	let sudo : &str = "5Gbvu58afWJabLsuUQ3Y8Y3KfJ6g9GGtabipjs5fZTSFp2DX";
	let sudo_stash : &str = "5G44CfowYQramZFYuYFGnHrCQ2LRJUmWa4o5c7GK279mkAhW";
	let _sudo_gran : &str = "5H5jpugVp68b8LS9nfhcWAYbs4VPEYyYumY3zy9FcKtWawps";

	// GOS AUTH PRIMARY, STASH, GRANDPA
	let auth_primary : &str = "5ELwfBR3LWP6gXhah8xT7o9yUPpMD62wMSkS8fKSiGA3nVUx";
	let auth_primary_stash: &str = "5DvEm6xrmYFbx1Mbh2AYxhJ8UBYRpBf5bCtXpnNcqyd2QJtP";
	let auth_primary_gran: &str = "5F4hFerb6ommCqeP5r3gGKTZ2rjCZhrkxNc8y5ZrnXtZSrqt";
	
	// GOS AUTH PRIMARY, STASH, GRANDPA
	let auth_secondary : &str = "5GNmbb63EhhYoAn8HjKkheNQSzmbcDPn11UMqyLKZCzLVVce";
	let auth_secondary_stash: &str = "5FEU3tuJsLhMXm411kPZKtsj7acijoj9MhPCtnMsWDALx894";
	let _auth_secondary_gran: &str = "5G2bGuzhdE5yxDp6dSXBMnM2vfttYJTrPHR6iVBXjTTUeeNs";
	
	// GOS OPS,STASH 
	let ops_primary : &str = "5EqHrsRmjb5NGjGndzLEB3CkYdawxhJ5D7so2edSRZtpHKQF";

	// GOS TEAM / FAUCET
	let team : &str = "5CAcPDwxRzzCJsFRipx53T7pBzkrtrxQ3JhwiWdxmNZzTgBX";
	
	genesis_config(
		// Initial PoA authorities
		vec![
			authority_keys_from_acct(auth_primary, auth_primary_stash, auth_primary_gran),
		],
		// Initial_nominators
		vec![],
		// Sudo Account
		Ss58Codec::from_string(sudo).unwrap(),
		// Pre-funded accounts
		vec![
			Ss58Codec::from_string(sudo).unwrap(),
			Ss58Codec::from_string(sudo_stash).unwrap(),
			Ss58Codec::from_string(auth_primary).unwrap(),
			Ss58Codec::from_string(auth_primary_stash).unwrap(),
			Ss58Codec::from_string(auth_secondary).unwrap(),
			Ss58Codec::from_string(auth_secondary_stash).unwrap(),

			Ss58Codec::from_string(ops_primary).unwrap(),
			Ss58Codec::from_string(team).unwrap(),
		].into(),
	)
}

/// Testnet config generator with (1) Validator
pub fn spectral_config() -> Result<ChainSpec, String> {
	// configure GOS currency symbol for the native coin
	let symbol_value = json!(SYMBOL_TEST);
	let token_decimals = json!(DECIMALS);

	let mut gos_props : Properties = Properties::new();
	gos_props.insert("tokenSymbol".to_string(), symbol_value);
	gos_props.insert("tokenDecimals".to_string(), token_decimals);

	Ok(ChainSpec::from_genesis(
		// Name
		"GOS Spectral",
		// ID
		"dev",
		ChainType::Development,
		// Spectral Genesis 
		spectral_config_genesis,
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("gos"),
		None,
		// Properties
		Some(gos_props),
		// Extensions
		Default::default(),
	))
}

/// Genesis Configuration Settings
fn live_config_genesis() -> RuntimeGenesisConfig {

	// GOS SUDO, STASH, GRANDPA
	let sudo : &str = "5DP6etCWFMVzD8PdYUyoqApqgX6wgk6Uyc7r5ZXYsFFZfTrj";
	let sudo_stash : &str = "5HH2EKHXztmMiYFdZAFqagAZcJv8XTpZaksemZYbtADUgzfa";
	let sudo_gran : &str = "5CBMCQh1Hi8uAPzCqw95Us7JEksZFnCGYttbb2YywaWCG8Y6";

	// GOS AUTH PRIMARY, STASH, GRANDPA
	let auth_primary : &str = "5GNBha7QhsQkt7rRjzULo1RpshiqkwD4hFvqNfe1x2jETFcH";
	let auth_primary_stash: &str = "5CSYC8SPwWgeQpZiRhxvwrehUoKujm8pqJZPw7d4buUL99VW";
	let auth_primary_gran: &str = "5CY8YfDyYtTXTZXUW75Vnu5b7gwc1knCiN9EzkRNwxfBX3vo";
	
	// GOS AUTH SECONDARY, STASH, GRANDPA
	let auth_secondary : &str = "5E1k39xTbrE5bcJpb8W9GHGi23hFnmdMt6LmoJsoQaTF63A3";
	let auth_secondary_stash: &str = "5DtqHYoTyyPxxEC2horUh7f7KPQUSBwdFuLiHy5kiewHxgZc";
	let auth_secondary_gran: &str = "5CAYMZTU4ZEwwvd1hTJZgFPyMbsaNgrtmiV6ZMUuuV7rSifi";
	
	// GOS AUTH TERTIARY, STASH, GRANDPA 
	let auth_tertiary : &str = "5G6Cc9Zw4XLPM3hJUVFLCfa4i6BGtN8LDdxzj7uKK9LQcu8m";
	let auth_tertiary_stash : &str = "5E27yuPHCLXDmXNaFwjKxzxXquFRAZ2PiFcEdbYsrv8XnKcQ";
	let auth_tertiary_gran : &str = "5HNMRGMfxHnrdCLjUZufXtdnkuZbjASvv9oXMBTjRaxqEY2y";

	// GOS OPS 
	let ops_primary : &str = "5DxwsfGtcfpH8163VEGFG9CUMQRs9497oWewkghP242KgSqi";

	// GOS TEAM
	let team_a : &str = "5CArGgcLdBMaF9yBKTh3rcYF66KgcxGuf7fG4BjWzKiaxtce";
	let team_b : &str = "5HQebPEwDigbHXMSPW1nq1KRuwZQehdBfVBn1cMrctw6FChf";
	

	genesis_config(
		// Initial PoA authorities
		vec![
			authority_keys_from_acct(sudo, sudo_stash, sudo_gran),
			authority_keys_from_acct(auth_primary, auth_primary_stash, auth_primary_gran),
			authority_keys_from_acct(auth_secondary, auth_secondary_stash, auth_secondary_gran),
			authority_keys_from_acct(auth_tertiary, auth_tertiary_stash, auth_tertiary_gran)
		],
		// Initial_nominators
		vec![],
		// Sudo Account
		Ss58Codec::from_string(sudo).unwrap(),
		// Pre-funded accounts
		vec![
			Ss58Codec::from_string(sudo).unwrap(),
			Ss58Codec::from_string(sudo_stash).unwrap(),
			Ss58Codec::from_string(auth_primary).unwrap(),
			Ss58Codec::from_string(auth_primary_stash).unwrap(),
			Ss58Codec::from_string(auth_secondary).unwrap(),
			Ss58Codec::from_string(auth_secondary_stash).unwrap(),
			Ss58Codec::from_string(ops_primary).unwrap(),
			Ss58Codec::from_string(team_a).unwrap(),
			Ss58Codec::from_string(team_b).unwrap(),
		].into(),
	)
}

/// Live config generator with (3) Validators
pub fn live_config() -> Result<ChainSpec, String> {
	// configure GOS currency symbol for the native coin
	let symbol_value = json!(SYMBOL);
	let token_decimals = json!(DECIMALS);

	let mut gos_props : Properties = Properties::new();
	gos_props.insert("tokenSymbol".to_string(), symbol_value);
	gos_props.insert("tokenDecimals".to_string(), token_decimals);

	Ok(ChainSpec::from_genesis(
		// Name
		"GOS Network",
		// ID
		"live",
		ChainType::Live,
		// Genesis 
		live_config_genesis,
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("gos"),
		None,
		// Properties
		Some(gos_props),
		// Extensions
		Default::default(),
	))
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> RuntimeGenesisConfig {
		genesis_config(
			vec![authority_keys_from_seed("Alice")],
			vec![],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
		)
	}

	/// Local testnet config (single validator - Alice).
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob).
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sp_tracing::try_init_simple();

		sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			let NewFullBase { task_manager, client, network, sync, transaction_pool, .. } =
				new_full_base(config, false, |_, _| ())?;
			Ok(sc_service_test::TestNetComponents::new(
				task_manager,
				client,
				network,
				sync,
				transaction_pool,
			))
		});
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		staging_testnet_config().build_storage().unwrap();
	}
}
