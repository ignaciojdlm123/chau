#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Address, BytesN, Env, Map, Vec, symbol_short
};

// Define the status enum for wills
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WillStatus {
    Draft,
    Active,
    Executed,
    Revoked,
}

// Define the condition struct for will execution
#[contracttype]
#[derive(Clone, Debug)]
pub struct Condition {
    condition_type: symbol_short!("type"),
    value: BytesN<32>,
}

// Define the struct for beneficiary data
#[contracttype]
#[derive(Clone, Debug)]
pub struct Beneficiary {
    address: Address,
    share: u32,  // Percentage share * 100 to handle decimals
    conditions: Vec<Condition>,
}

// Define the main will data structure
#[contracttype]
#[derive(Clone, Debug)]
pub struct WillData {
    owner: Address,
    content_hash: BytesN<32>,
    status: WillStatus,
    beneficiaries: Vec<Beneficiary>,
    execution_conditions: Vec<Condition>,
    created_at: u64,
    last_modified: u64,
}

// Define the contract storage
#[contract]
pub struct WillRegistry {
    owner: Address,
    wills: Map<BytesN<32>, WillData>,
}

#[contractimpl]
impl WillRegistry {
    // Initialize the contract
    pub fn initialize(env: Env, owner: Address) -> Self {
        if env.storage().instance().has(&symbol_short!("init")) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&symbol_short!("init"), true);
        
        Self {
            owner,
            wills: Map::new(&env),
        }
    }

    // Create a new will
    pub fn create_will(
        env: Env,
        owner: Address,
        content_hash: BytesN<32>,
        beneficiaries: Vec<Beneficiary>,
        execution_conditions: Vec<Condition>,
    ) -> BytesN<32> {
        // Verify the caller is the owner
        owner.require_auth();

        // Generate a unique will ID using the env timestamp and owner address
        let timestamp = env.ledger().timestamp();
        let will_id = env.crypto().sha256(
            &[&owner.to_array()[..], &timestamp.to_be_bytes()].concat()
        );

        // Create the will data
        let will_data = WillData {
            owner: owner.clone(),
            content_hash,
            status: WillStatus::Draft,
            beneficiaries,
            execution_conditions,
            created_at: timestamp,
            last_modified: timestamp,
        };

        // Validate beneficiary shares total to 100%
        Self::validate_beneficiary_shares(&will_data.beneficiaries);

        // Store the will
        self.wills.set(will_id.clone(), will_data);

        // Return the will ID
        will_id
    }

    // Activate a will (change from Draft to Active)
    pub fn activate_will(env: Env, will_id: BytesN<32>) {
        let mut will = self.get_will(will_id.clone());
        
        // Verify the caller is the owner
        will.owner.require_auth();

        // Verify the will is in Draft status
        if will.status != WillStatus::Draft {
            panic!("Will must be in Draft status to activate");
        }

        // Update the will status
        will.status = WillStatus::Active;
        will.last_modified = env.ledger().timestamp();
        
        // Store the updated will
        self.wills.set(will_id, will);
    }

    // Update an existing will
    pub fn update_will(
        env: Env,
        will_id: BytesN<32>,
        new_content_hash: BytesN<32>,
        new_beneficiaries: Vec<Beneficiary>,
        new_execution_conditions: Vec<Condition>,
    ) {
        let mut will = self.get_will(will_id.clone());
        
        // Verify the caller is the owner
        will.owner.require_auth();

        // Verify the will is not already executed or revoked
        if will.status == WillStatus::Executed || will.status == WillStatus::Revoked {
            panic!("Cannot update an executed or revoked will");
        }

        // Validate new beneficiary shares
        Self::validate_beneficiary_shares(&new_beneficiaries);

        // Update the will data
        will.content_hash = new_content_hash;
        will.beneficiaries = new_beneficiaries;
        will.execution_conditions = new_execution_conditions;
        will.last_modified = env.ledger().timestamp();
        
        // Store the updated will
        self.wills.set(will_id, will);
    }

    // Revoke a will
    pub fn revoke_will(env: Env, will_id: BytesN<32>) {
        let mut will = self.get_will(will_id.clone());
        
        // Verify the caller is the owner
        will.owner.require_auth();

        // Verify the will is not already executed
        if will.status == WillStatus::Executed {
            panic!("Cannot revoke an executed will");
        }

        // Update the will status
        will.status = WillStatus::Revoked;
        will.last_modified = env.ledger().timestamp();
        
        // Store the updated will
        self.wills.set(will_id, will);
    }

    // Get will data
    pub fn get_will(&self, will_id: BytesN<32>) -> WillData {
        self.wills.get(will_id.clone()).unwrap_or_else(|| panic!("Will not found"))
    }

    // Helper function to validate beneficiary shares
    fn validate_beneficiary_shares(beneficiaries: &Vec<Beneficiary>) {
        let total_shares: u32 = beneficiaries
            .iter()
            .map(|b| b.share)
            .sum();

        if total_shares != 10000 {  // 100% * 100 to handle decimals
            panic!("Total beneficiary shares must equal 100%");
        }
    }

    // Check if conditions for will execution are met
    pub fn check_execution_conditions(
        env: Env,
        will_id: BytesN<32>,
    ) -> bool {
        let will = self.get_will(will_id);
        
        // Implement your condition checking logic here
        // This is a placeholder that always returns true
        true
    }

    // Execute a will
    pub fn execute_will(env: Env, will_id: BytesN<32>) {
        let mut will = self.get_will(will_id.clone());
        
        // Verify that execution conditions are met
        if !self.check_execution_conditions(env.clone(), will_id.clone()) {
            panic!("Execution conditions not met");
        }

        // Verify the will is Active
        if will.status != WillStatus::Active {
            panic!("Will must be Active to execute");
        }

        // Update the will status
        will.status = WillStatus::Executed;
        will.last_modified = env.ledger().timestamp();
        
        // Store the updated will
        self.wills.set(will_id, will);

        // Here you would typically emit events or trigger token transfers
        // to beneficiaries based on their shares
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[test]
    fn test_create_will() {
        let env = Env::default();
        let owner = Address::random(&env);
        
        let contract = WillRegistry::initialize(
            env.clone(),
            owner.clone(),
        );

        let content_hash = BytesN::from_array(
            &env,
            &env.crypto().sha256(&[1, 2, 3]),
        );

        let beneficiary = Beneficiary {
            address: Address::random(&env),
            share: 10000,  // 100%
            conditions: Vec::new(&env),
        };

        let beneficiaries = Vec::from_array(&env, [beneficiary]);
        let execution_conditions = Vec::new(&env);

        let will_id = contract.create_will(
            env.clone(),
            owner.clone(),
            content_hash.clone(),
            beneficiaries,
            execution_conditions,
        );

        let stored_will = contract.get_will(will_id);
        assert_eq!(stored_will.owner, owner);
        assert_eq!(stored_will.content_hash, content_hash);
        assert_eq!(stored_will.status, WillStatus::Draft);
    }
}
