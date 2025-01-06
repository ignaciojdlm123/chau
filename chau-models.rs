use soroban_sdk::{
    contracttype, Address, BytesN, Env, Map, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WillStatus {
    Draft,
    Active,
    Executed,
    Revoked,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub parameters: Map<BytesN<32>, BytesN<32>>,
    pub required_signers: Vec<Address>,
    pub oracle_address: Option<Address>,
    pub verification_data: Option<BytesN<32>>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionType {
    TimeAfter,
    TimeBefore,
    MultiSigRequired,
    ExternalOracle,
    AgeRequirement,
    LocationBased,
    TokenBalance,
    CodicilAttached,
    LegalApproval,
    MajorityConsent,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Beneficiary {
    pub address: Address,
    pub share: u32,
    pub conditions: Vec<Condition>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct WillData {
    pub owner: Address,
    pub content_hash: BytesN<32>,
    pub status: WillStatus,
    pub beneficiaries: Vec<Beneficiary>,
    pub execution_conditions: Vec<Condition>,
    pub created_at: u64,
    pub last_modified: u64,
}
