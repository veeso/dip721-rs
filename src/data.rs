use candid::{CandidType, Deserialize, Int, Nat, Principal};
use serde::Serialize;

/// NFT token identifier
pub type TokenIdentifier = Nat;

/// Properties value representation for a token
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64), // motoko only support f64
    NestedContent(Vec<(String, GenericValue)>),
}

/// Metadata for a DIP721 token
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TokenMetadata {
    pub approved_at: Option<u64>,
    pub approved_by: Option<Principal>,
    pub burned_at: Option<u64>,
    pub burned_by: Option<Principal>,
    pub is_burned: bool,
    pub minted_at: u64,
    pub minted_by: Principal,
    pub operator: Option<Principal>,
    pub owner: Option<Principal>,
    pub properties: Vec<(String, GenericValue)>,
    pub token_identifier: TokenIdentifier,
    pub transferred_at: Option<u64>,
    pub transferred_by: Option<Principal>,
}

#[cfg(feature = "ic-stable-structures")]
impl ic_stable_structures::Storable for TokenMetadata {
    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        use candid::Encode;
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        use candid::Decode;
        Decode!(&bytes, Self).unwrap()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    #[cfg(feature = "ic-stable-structures")]
    fn test_should_encode_and_decode_nft() {
        use ic_stable_structures::Storable as _;
        let token_metadata = TokenMetadata {
            approved_at: Some(1),
            approved_by: Some(Principal::management_canister()),
            burned_at: Some(3),
            burned_by: Some(Principal::management_canister()),
            is_burned: true,
            minted_at: 5,
            minted_by: Principal::management_canister(),
            operator: Some(Principal::management_canister()),
            owner: Some(Principal::management_canister()),
            properties: vec![(
                "key".to_string(),
                GenericValue::TextContent("value".to_string()),
            )],
            token_identifier: 9_u64.into(),
            transferred_at: Some(10),
            transferred_by: Some(Principal::management_canister()),
        };

        let bytes = token_metadata.to_bytes();
        let decoded = TokenMetadata::from_bytes(bytes);

        assert_eq!(token_metadata, decoded);
    }
}
