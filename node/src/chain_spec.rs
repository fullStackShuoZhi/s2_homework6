use hex_literal::hex;

use node_template_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY,BABE_GENESIS_EPOCH_CONFIG,SessionConfig, StakingConfig, SessionKeys,
	constants::currency::*, StakerStatus, MaxNominations,ImOnlineConfig
};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sc_telemetry::TelemetryEndpoints;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use node_primitives::*;

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online}
}
/// Generate an Babe authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
                vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
                vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
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
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Substrate Stencil",
		"stencil_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	let wasm_binary = WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	);

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5EHjoBVGQm4CNbdoeaozpAKGp9TTrfRuADvQ65kyiABZhYPk
			hex!["62692ff7965729b21a10862da513beb42b7da54f321e35e2147983e1e53dc935"].into(),
			// 5DSShm3qptXjE5aK7aUoVCQ7ScgCwt8wbH7MzgNwtRg4FPJZ
			hex!["3cd09eecf6faa579ff49a5bb8175c02244da1151cfa75b8b3fc9dcb15b4b281d"].into(),
			// 5DRQRDWEabzp8BkYjDWRmzBiBqc6DyyPKDcuogXXXgR9cVfr
			hex!["3c05aebeb9e9740fc49fba44d36958cfebaed238a15e5d215d20c3b858c58d16"]
				.unchecked_into(),
			// 5G8WWBAq4gpFLEnQjsxPkfYRdXNs3UkaeR8QG39HVjub8agw
			hex!["b3d7b25de30f345bfb40e0fb78f86acc36a7edc045615d5dee2cb9539faa8219"]
				.unchecked_into(),
			// 5Fvbhw7qHqrUXfErS8afAzPvpnbEk6BCXxMzoFTYk5Lg2dmD
			hex!["aac2440bfa090b89b151bd32e664cf3a77646040d58beeca4c66a8727d78fa4c"]
				.unchecked_into(),
		),
		(
			// 5FsJoADMCmUQqmprSSqnyLPj7KAz7Kym6AzWbXMyEza7A5XH
			hex!["a83f9b156daa23ac07dd3361514d1b9f1674904d35ce8ab422bc8e1e12dac70b"].into(),
			// 5GE6M2FBBChfGfatFvRmWSgJrvSuxVYB2HNA13Fb5EFMpjst
			hex!["b819d8c01cbc46e23d9b79f7654f704a828fa1946bc8a97f56889daade1ced4e"].into(),
			// 5GxzB3oemXt3XskegqpVAMfCNQf1xxgSEwUz5tt97Xodg7fZ
			hex!["d8d09f57c3ba6ab33c1ffda74aa70af40eade733d77c642a7159e2621c591203"]
				.unchecked_into(),
			// 5EaM6zor3sPnqfYLM1CRu1RFPsyMkNa1B1r3J4itBSLs8mx8
			hex!["6f13f7e727ef6b4094b346e351e66242b51fbbb6a2eac532b55389f1314d2d11"]
				.unchecked_into(),
			// 5GbuPR9jeuLuzVyyXQ8i4UF22G7HTG9RMencHkqNPfQMgLix
			hex!["c8bc5af37ae86784b073d2b88133cae60c268c31a1c2b108c92d3d16fc3a7d77"]
				.unchecked_into(),
		),
		(
			// 5EC9ZatAJNcG1zrbkuJJjwfe4k2Dbs3XmDvk4MBjKanw6tpL
			hex!["5e25b78d7ef73fb03c48b5550c7762f2fffaff54ef6cac0d670157cf2ba18563"].into(),
			// 5CcwzmrQg71nAQX8XpFKXojxF7GUHY5C74c4HeXdxa5uVFxK
			hex!["1897739a555a3ffc548045b2d3580510e9d30e4529d7b92bc35da4421200d160"].into(),
			// 5Cfr34yFa7ZXKpVJZUG9BrxUzoEGewM15rgmJUNTKAGdAqhR
			hex!["1acd1f89ba95f376393668a0461f5a77fa872370ce79ec3eefdd543b88af5b11"]
				.unchecked_into(),
			// 5FmbyAA9Qj34wGbKRUG6ECDHx7nNqeJe9jz3q1nJnaMDk25b
			hex!["a3e5f0d91041642f5c82b75ba6ef5fbd429463797e66e813a816251a1a64e3ff"]
				.unchecked_into(),
			// 5FUDFLosMyMhiV1cuD4JvxMPfhtTGDS9Ta8eA1whcDuLUA3u
			hex!["96a248883659efafd966e84a71c8387ac05c0838b34469f5e10e00f9c5da4c26"]
				.unchecked_into(),
		),
		(
			// 5FZcArDEk2ddwCRDVn6NzuL68X6cAcoqAxZ2kkLLMjG3wBgb
			hex!["9abfacaa81504b72ed3d0e4379e6604320b394cb5dc75f89bac64fc0798a901a"].into(),
			// 5F1dcFSJkE9RweJqsjKbH8VFdpo1Pibo6s5m4UxWs1zqH59E
			hex!["825ca97d686833d181f76235eab34d3c415947a3363f559db3d966616291cb53"].into(),
			// 5GFC1BkjF1trUAJaZZuadE7KHseznFS4fiywewXewe1iw6xv
			hex!["b8f0237be31bf5ac9c2390d1e62ca24e276ca77aeb0a3011b6c3ce47c4a05945"]
				.unchecked_into(),
			// 5FXzQX5s63RAAuRi8q4XoFa9VPaCLwa5Np3fbWgyXFUrPDiR
			hex!["99840922a08e2473e9e63718f9e562cfe391eb3ca5c4f6b22d03a5fa3f5148de"]
				.unchecked_into(),
			// 5F3veQDYtog6CttMtkvGscE5RswzN66K9KaaXLSDtFCFWwnH
			hex!["841c813d8917ef3164dc3a8734f7f09713e83cf17eec5a83df5bb1c4588c3f28"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5EsZbZbS8WntKbCqpXji9bztnpbBNbXmdsDfsfQky5w53jLz
		"7c35319a911d7729df1ebff84617747c29b889c96dbbf5696161f933a9682000"
	]
		.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		wasm_binary,
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts,
		true,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
    initial_nominators: Vec<AccountId>,
	root_key: AccountId,
    mut endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
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
    const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
    const STASH: Balance = ENDOWMENT / 1000;
    let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
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

    GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(),x.4.clone()),
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
        im_online: ImOnlineConfig { keys: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
