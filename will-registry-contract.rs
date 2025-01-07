use soroban_sdk::{contract, contractimpl, Address, Env, BytesN, Vec, Map, Symbol};

#[contract]
pub struct WillRegistry;

#[derive(Clone)]
pub enum WillStatus {
    Active,
    Executed,
    Revoked,
}

#[derive(Clone)]
pub struct WillData {
    owner: Address,
    content_hash: BytesN<32>,
    status: WillStatus,
    beneficiaries: Vec<Address>,
    last_modified: u64,
}

#[derive(Clone)]
pub struct Condition {
    condition_type: Symbol,
    value: BytesN<32>,
}

#[contractimpl]
impl WillRegistry {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::short("admin"), &admin);
    }

    pub fn create_will(
        env: Env,
        owner: Address,
        content_hash: BytesN<32>,
        beneficiaries: Vec<Address>,
    ) -> BytesN<32> {
        // Verify the caller is the owner
        owner.require_auth();

        // Generate a unique will ID using the environment
        let will_id = env.crypto().sha256(&env.current_contract_address().into_val(&env));

        // Create the will data
        let will_data = WillData {
            owner: owner.clone(),
            content_hash,
            status: WillStatus::Active,
            beneficiaries,
            last_modified: env.ledger().timestamp(),
        };

        // Store the will data
        env.storage().instance().set(&will_id, &will_data);

        // Return the will ID
        will_id
    }

    pub fn update_will(
        env: Env,
        will_id: BytesN<32>,
        new_content_hash: BytesN<32>,
        new_beneficiaries: Option<Vec<Address>>,
    ) {
        // Load existing will data
        let mut will_data: WillData = env.storage().instance().get(&will_id).unwrap();

        // Verify the caller is the owner
        will_data.owner.require_auth();

        // Update will data
        will_data.content_hash = new_content_hash;
        if let Some(beneficiaries) = new_beneficiaries {
            will_data.beneficiaries = beneficiaries;
        }
        will_data.last_modified = env.ledger().timestamp();

        // Store updated will data
        env.storage().instance().set(&will_id, &will_data);
    }

    pub fn execute_will(env: Env, will_id: BytesN<32>) {
        // Load existing will data
        let mut will_data: WillData = env.storage().instance().get(&will_id).unwrap();

        // Only admin can execute wills
        let admin: Address = env.storage().instance().get(&Symbol::short("admin")).unwrap();
        admin.require_auth();

        // Verify will is active
        match will_data.status {
            WillStatus::Active => {},
            _ => panic!("Will is not in active status"),
        }

        // Update status to executed
        will_data.status = WillStatus::Executed;
        will_data.last_modified = env.ledger().timestamp();

        // Store updated will data
        env.storage().instance().set(&will_id, &will_data);
    }

    pub fn get_will_data(env: Env, will_id: BytesN<32>) -> WillData {
        env.storage().instance().get(&will_id).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[test]
    fn test_will_lifecycle() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WillRegistry);
        let client = WillRegistryClient::new(&env, &contract_id);

        // Test addresses
        let admin = Address::random(&env);
        let owner = Address::random(&env);
        let beneficiary = Address::random(&env);

        // Initialize contract
        client.initialize(&admin);

        // Create will
        let content_hash = BytesN::from_array(&env, &[0; 32]);
        let beneficiaries = vec![&env, beneficiary.clone()];
        
        // Create will with owner authentication
        env.as_contract(&owner, || {
            client.create_will(&owner, &content_hash, &beneficiaries)
        });

        // TODO: Add more test cases for update and execute operations
    }
}
