use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::GovActionId;
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)] // #[derive(PartialOrd, Ord] somehow missing
pub struct GovActionIdWrapper {
    #[n(0)]
    pub pallas_gov_action_id: GovActionId,
}

impl GovActionIdWrapper {
    pub fn new(transaction_id: &str, index: u32) -> Result<Self, String> {
        let digest: Hash<32> = transaction_id
            .parse()
            .map_err(|_| "Invalid transaction id length".to_string())?;

        Ok(Self {
            pallas_gov_action_id: GovActionId {
                transaction_id: digest,
                action_index: index,
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let gov_action_id = GovActionId::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self {
            pallas_gov_action_id: gov_action_id,
        })
    }

    pub fn into_inner(&self) -> GovActionId {
        self.pallas_gov_action_id.clone()
    }
}
