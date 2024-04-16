use candid::{CandidType, Deserialize, Nat, Principal};
use thiserror::Error;

/// Metadata for a DIP721 canister
#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub created_at: u64,
    pub custodians: Vec<Principal>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub upgraded_at: u64,
}

/// Canister stats
#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Stats {
    pub cycles: Nat,
    pub total_supply: Nat,
    pub total_transactions: Nat,
    pub total_unique_holders: Nat,
}

/// Supported interfaces for a DIP721 canister
#[derive(CandidType, PartialEq, Eq, Debug, Deserialize, Clone, Copy)]
pub enum SupportedInterface {
    Approval,
    Burn,
    Mint,
    TransactionHistory,
}

/// Represent an NFT error to return via API
#[derive(CandidType, Debug, Deserialize, Clone, PartialEq, Eq, Error)]
pub enum NftError {
    #[error("self transfer is not allowed")]
    SelfTransfer,
    #[error("token not found")]
    TokenNotFound,
    #[error("transaction not found")]
    TxNotFound,
    #[error("not approved")]
    SelfApprove,
    #[error("operator not found")]
    OperatorNotFound,
    #[error("unauthorized owner")]
    UnauthorizedOwner,
    #[error("unauthorized operator")]
    UnauthorizedOperator,
    #[error("NFT existed")]
    ExistedNFT,
    #[error("owner not found")]
    OwnerNotFound,
    #[error("{0}")]
    Other(String),
}
