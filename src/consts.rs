use solana_program::{keccak::Hash, pubkey, pubkey::Pubkey};

/// The unix timestamp after which mining can begin.
pub const START_AT: i64 = 1712070600;

/// The reward rate to intialize the program with.
pub const INITIAL_REWARD_RATE: u64 = 10u64.pow(3u32);

/// The mining difficulty to initialize the program with.
pub const INITIAL_DIFFICULTY: Hash = Hash::new_from_array([
    0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
]);

/// The decimal precision of the ORE token.
/// Using SI prefixes, the smallest indivisible unit of ORE is a nanoORE.
/// 1 nanoORE = 0.000000001 ORE = one billionth of an ORE
pub const TOKEN_DECIMALS: u8 = 9;

/// One ORE token, denominated in units of nanoORE.
pub const ONE_ORE: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// Capped supply at 21M.
pub const TOTAL_SUPPLY: u64 = 1000000 * ONE_ORE;

/// The duration of an epoch, in units of seconds.
pub const EPOCH_DURATION: i64 = 60;

/// The target quantity of ORE to be mined per epoch, in units of nanoORE.
/// Inflation rate ≈ 1 ORE / epoch (min 0, max 2)
pub const TARGET_EPOCH_REWARDS: u64 = ONE_ORE;

/// The maximum quantity of ORE that can be mined per epoch, in units of nanoORE.
pub const MAX_EPOCH_REWARDS: u64 = ONE_ORE.saturating_mul(2);

/// The quantity of ORE each bus is allowed to issue per epoch.
pub const BUS_EPOCH_REWARDS: u64 = MAX_EPOCH_REWARDS.saturating_div(BUS_COUNT as u64);

/// The number of bus accounts, for parallelizing mine operations.
pub const BUS_COUNT: usize = 8;

/// The smoothing factor for reward rate changes. The reward rate cannot change by more or less
/// than a factor of this constant from one epoch to the next.
pub const SMOOTHING_FACTOR: u64 = 2;

// Assert MAX_EPOCH_REWARDS is evenly divisible by BUS_COUNT.
static_assertions::const_assert!(
    (MAX_EPOCH_REWARDS / BUS_COUNT as u64) * BUS_COUNT as u64 == MAX_EPOCH_REWARDS
);

/// The seed of the bus account PDA.
pub const BUS: &[u8] = b"bus";

/// The seed of the metadata account PDA.
pub const METADATA: &[u8] = b"metadata";

/// The seed of the mint account PDA.
pub const MINT: &[u8] = b"mint";

/// The seed of proof account PDAs.
pub const PROOF: &[u8] = b"proof";

/// The seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// The name for token metadata.
pub const METADATA_NAME: &str = "Powx";

/// The ticker symbol for token metadata.
pub const METADATA_SYMBOL: &str = "POWX";

/// The uri for token metdata.
pub const METADATA_URI: &str = "https://powx.ai/metadata.json";

/// Noise for deriving the mint PDA.
pub const MINT_NOISE: [u8; 16] = [
    200, 99, 199, 116, 3, 68, 162, 90, 175, 2, 164, 172, 3, 229, 8, 102,
];

/// The addresses of the bus accounts.
pub const BUS_ADDRESSES: [Pubkey; BUS_COUNT] = [
    pubkey!("2dUPVK4mrznAwQds4BAqFzrmPpHMFcvtgomDWhuNkgaa"),
    pubkey!("ETrkrqKuvKYDK1knaqHbTeVm2j1T7kLXcnZYbJXu2i24"),
    pubkey!("4Ba5bLxPHW5FrDaE7cLK8c7q7zPTz7zfK115PjEBAH9Z"),
    pubkey!("6pphc5xoHV8ckxE4FQaDK5Rz71FUcXpwG9WLzUvSXaYe"),
    pubkey!("3ds5oRfjbhKTL1KV4JjYBCXkQCqWZCTz11qhshAnzbgi"),
    pubkey!("7nbZEfGtdg5toqJJDvvvfCZ9biyMZTLdwB8bf2dbw5fw"),
    pubkey!("EyA5ZM5wUtphGgupD7QLEBazCE6oopNHrvaSvzzSdopi"),
    pubkey!("7Zq9AsHRWPBPLs7xpiLrpxjgoexLnKKu1xPHWipJgkiS"),
];

/// The address of the mint metadata account.
pub const METADATA_ADDRESS: Pubkey = pubkey!("5kmZ4gT7sYWJJPYfc5VqeThoVDPmUz28cKNtwZhhPJ8x");

/// The address of the mint account.
pub const MINT_ADDRESS: Pubkey = pubkey!("powvaMAzydGVFgCcAvyhEyBcBHEHLMVis3wLpWNW8pS");

/// The address of the treasury account.
pub const TREASURY_ADDRESS: Pubkey = pubkey!("CmHjCfgvSjnJQGxYQ9szvi57y7CEhvDP2LEPCeHysksE");
