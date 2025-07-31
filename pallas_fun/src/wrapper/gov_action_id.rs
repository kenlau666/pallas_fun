pub use pallas::crypto::hash::Hash;

use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::GovActionId;
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)] // #[derive(PartialOrd, Ord] somehow missing
pub struct GovActionIdWrapper {
    #[n(0)]
    pub pallas_gov_action_id: GovActionId,
}

impl GovActionIdWrapper {
    pub fn new(transaction_id: &str, index: u32) -> Self {
        let digest: Hash<32> = transaction_id
            .parse()
            .expect("Invalid transaction id length");

        Self {
            pallas_gov_action_id: GovActionId {
                transaction_id: digest,
                action_index: index,
            },
        }
    }

    pub fn encode(self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(self, hex_string: String) -> Self {
        Self {
            pallas_gov_action_id: GovActionId::decode_fragment(&hex::decode(hex_string).unwrap())
                .unwrap(),
        }
    }

    pub fn into_inner(self) -> GovActionId {
        self.pallas_gov_action_id
    }
}
