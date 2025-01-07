use soroban_sdk::{contract, contractimpl, Address, Env, BytesN, Map, Symbol};

#[contract]
pub struct TokenManager;

#[derive(Clone)]
pub struct TokenData {
    will_id: BytesN<32>,
    owner: Address,
    metadata: BytesN<32>,
    issued_at: u64,
}

#[contractimpl]
impl TokenManager {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::short("admin"), &admin);
    }

    pub fn mint_token(
        env: Env,
        to: Address,
        will_id: BytesN<32>,
        metadata: BytesN<32>,
    ) -> BytesN<32> {
        // Only admin can mint tokens
        let admin: Address = env.storage().instance().get(&Symbol::short("admin")).unwrap();
        admin.require_auth();

        // Generate token ID
        let token_id = env.crypto().sha256(&will_id);

        // Create token data
        let token_data = TokenData {
            will_id,
            owner: to.clone(),
            metadata,
            issued_at: env.ledger().timestamp(),
        };

        // Store token data
        env.storage().instance().set(&token_id, &token_data);

        token_id
    }

    pub fn transfer_token(
        env: Env,
        from: Address,
        to: Address,
        token_id: BytesN<32>,
    ) {
        // Load token data
        let mut token_data: TokenData = env.storage().instance().get(&token_id).unwrap();

        // Verify the caller is the current owner
        from.require_auth();
        assert_eq!(token_data.owner, from, "Caller is not token owner");

        // Update owner
        token_data.owner = to;

        // Store updated token data
        env.storage().instance().set(&token_id, &token_data);
    }

    pub fn get_token_data(env: Env, token_id: BytesN<32>) -> TokenData {
        env.storage().instance().get(&token_id).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[test]
    fn test_token_lifecycle() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenManager);
        let client = TokenManagerClient::new(&env, &contract_id);

        // Test addresses
        let admin = Address::random(&env);
        let user = Address::random(&env);

        // Initialize contract
        client.initialize(&admin);

        // Test data
        let will_id = BytesN::from_array(&env, &[0; 32]);
        let metadata = BytesN::from_array(&env, &[1; 32]);

        // Mint token with admin authentication
        let token_id = env.as_contract(&admin, || {
            client.mint_token(&user, &will_id, &metadata)
        });

        // TODO: Add more test cases for transfer operations
    }
}
