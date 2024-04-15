use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::GenericValue;

/// Transaction event
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TxEvent {
    pub caller: Principal,
    pub details: Vec<(String, GenericValue)>,
    pub operation: String,
    pub time: u64,
}

#[cfg(feature = "ic-stable-structures")]
impl ic_stable_structures::Storable for TxEvent {
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
    fn test_should_encode_and_decode_tx_event() {
        use ic_stable_structures::Storable as _;
        let tx_event = TxEvent {
            caller: Principal::management_canister(),
            details: vec![(
                "key".to_string(),
                GenericValue::TextContent("value".to_string()),
            )],
            operation: "operation".to_string(),
            time: 1,
        };
        let bytes = tx_event.to_bytes();
        let decoded = TxEvent::from_bytes(bytes);
        assert_eq!(tx_event, decoded);
    }
}
