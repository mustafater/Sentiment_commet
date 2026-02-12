#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Vec, log};

/// A sampled negative comment stored on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct NegativeComment {
    /// MongoDB comment ID (hex string)
    pub comment_id: String,
    /// Sentiment score (0-100, lower = more negative)
    pub score: u32,
    /// Hash/truncated content for verification
    pub content_hash: String,
    /// Timestamp of submission (ledger sequence as proxy)
    pub timestamp: u64,
}

/// Storage keys
#[contracttype]
pub enum DataKey {
    /// The reservoir array of sampled negative comments
    Reservoir,
    /// Total number of negative comments seen
    TotalCount,
    /// Maximum reservoir size (k)
    MaxSize,
    /// Contract admin
    Admin,
}

const DEFAULT_K: u32 = 10;

#[contract]
pub struct NegativeSamplerContract;

#[contractimpl]
impl NegativeSamplerContract {
    /// Initialize the contract with an admin and optional reservoir size.
    pub fn initialize(env: Env, admin: soroban_sdk::Address, max_size: u32) {
        // Only allow initialization once
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        let k = if max_size == 0 { DEFAULT_K } else { max_size };

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::MaxSize, &k);
        env.storage().instance().set(&DataKey::TotalCount, &0u64);

        let empty_reservoir: Vec<NegativeComment> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Reservoir, &empty_reservoir);

        log!(&env, "NegativeSampler initialized with k={}", k);
    }

    /// Submit a negative comment for reservoir sampling.
    ///
    /// Uses Algorithm R (Vitter's reservoir sampling):
    /// - If reservoir has < k items, always add
    /// - If reservoir has k items, replace a random item with probability k/n
    ///
    /// This ensures a fair random sample of all seen negative comments,
    /// bounded by k entries to keep gas costs predictable.
    pub fn submit_negative(
        env: Env,
        comment_id: String,
        score: u32,
        content_hash: String,
    ) {
        let k: u32 = env.storage().instance().get(&DataKey::MaxSize)
            .unwrap_or(DEFAULT_K);
        let mut total: u64 = env.storage().instance().get(&DataKey::TotalCount)
            .unwrap_or(0);
        let mut reservoir: Vec<NegativeComment> = env.storage().instance()
            .get(&DataKey::Reservoir)
            .unwrap_or_else(|| Vec::new(&env));

        total += 1;

        let new_comment = NegativeComment {
            comment_id,
            score,
            content_hash,
            timestamp: env.ledger().sequence() as u64,
        };

        if (reservoir.len() as u32) < k {
            // Reservoir not full — always add
            reservoir.push_back(new_comment);
            log!(&env, "Added to reservoir (slot {})", reservoir.len());
        } else {
            // Reservoir full — replace with probability k/n
            // Generate random index in [0, total)
            let random_val = env.prng().gen_range::<u64>(0..total);

            if random_val < k as u64 {
                // Replace the item at random_val index
                let replace_idx = random_val as u32;
                reservoir.set(replace_idx, new_comment);
                log!(&env, "Replaced slot {} (n={})", replace_idx, total);
            } else {
                log!(&env, "Discarded (n={}, probability was {}/{})", total, k, total);
            }
        }

        // Persist
        env.storage().instance().set(&DataKey::Reservoir, &reservoir);
        env.storage().instance().set(&DataKey::TotalCount, &total);

        // Extend TTL to keep data alive
        env.storage().instance().extend_ttl(50000, 100000);
    }

    /// Get the current reservoir sample.
    pub fn get_sample(env: Env) -> Vec<NegativeComment> {
        env.storage().instance()
            .get(&DataKey::Reservoir)
            .unwrap_or_else(|| Vec::new(&env))
    }

    /// Get statistics about the sampling.
    pub fn get_stats(env: Env) -> (u64, u32, u32) {
        let total: u64 = env.storage().instance().get(&DataKey::TotalCount).unwrap_or(0);
        let k: u32 = env.storage().instance().get(&DataKey::MaxSize).unwrap_or(DEFAULT_K);
        let reservoir: Vec<NegativeComment> = env.storage().instance()
            .get(&DataKey::Reservoir)
            .unwrap_or_else(|| Vec::new(&env));
        let current_size = reservoir.len() as u32;

        // Returns (total_seen, max_reservoir_size, current_reservoir_size)
        (total, k, current_size)
    }

    /// Reset the reservoir (admin only).
    pub fn reset(env: Env, admin: soroban_sdk::Address) {
        let stored_admin: soroban_sdk::Address = env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Not initialized");

        admin.require_auth();
        if admin != stored_admin {
            panic!("Unauthorized");
        }

        let empty: Vec<NegativeComment> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Reservoir, &empty);
        env.storage().instance().set(&DataKey::TotalCount, &0u64);

        log!(&env, "Reservoir reset by admin");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Env, Address};

    #[test]
    fn test_initialize_and_submit() {
        let env = Env::default();
        let contract_id = env.register(NegativeSamplerContract, ());
        let client = NegativeSamplerContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin, &5);

        // Submit 3 comments (below k=5, all should be added)
        for i in 0..3 {
            client.submit_negative(
                &String::from_str(&env, &format!("comment_{}", i)),
                &(20 + i),
                &String::from_str(&env, &format!("hash_{}", i)),
            );
        }

        let sample = client.get_sample();
        assert_eq!(sample.len(), 3);

        let stats = client.get_stats();
        assert_eq!(stats.0, 3); // total_seen
        assert_eq!(stats.1, 5); // max_size
        assert_eq!(stats.2, 3); // current_size
    }

    #[test]
    fn test_reservoir_fills_and_caps() {
        let env = Env::default();
        let contract_id = env.register(NegativeSamplerContract, ());
        let client = NegativeSamplerContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin, &3); // k=3

        // Submit 10 comments
        for i in 0..10 {
            client.submit_negative(
                &String::from_str(&env, &format!("c{}", i)),
                &(10 + i),
                &String::from_str(&env, &format!("h{}", i)),
            );
        }

        let sample = client.get_sample();
        let stats = client.get_stats();

        // Reservoir should never exceed k=3
        assert!(sample.len() <= 3);
        assert_eq!(stats.0, 10); // total_seen = 10
        assert_eq!(stats.1, 3);  // max_size = 3
    }

    #[test]
    fn test_reset() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(NegativeSamplerContract, ());
        let client = NegativeSamplerContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin, &5);

        client.submit_negative(
            &String::from_str(&env, "test_comment"),
            &15,
            &String::from_str(&env, "test_hash"),
        );

        let stats_before = client.get_stats();
        assert_eq!(stats_before.0, 1);

        client.reset(&admin);

        let stats_after = client.get_stats();
        assert_eq!(stats_after.0, 0);
        assert_eq!(stats_after.2, 0);
    }
}
