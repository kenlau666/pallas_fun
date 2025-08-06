use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
pub use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::{Fragment, TransactionInput};
use serde::{Deserialize, Serialize};

use crate::utils::IntoInner;

#[derive(
    Serialize,
    Deserialize,
    Encode,
    Decode,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    std::hash::Hash,
)]
pub struct TransactionInputWrapper {
    #[n(0)]
    pub inner: TransactionInput,
}

impl TransactionInputWrapper {
    pub fn new(transaction_id: &str, index: u64) -> Result<Self, String> {
        let digest: Hash<32> = transaction_id
            .parse()
            .map_err(|_| "Invalid transaction id length".to_string())?;

        Ok(Self {
            inner: TransactionInput {
                transaction_id: digest,
                index: index,
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let tx_input = TransactionInput::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self { inner: tx_input })
    }
}

impl IntoInner<TransactionInput> for TransactionInputWrapper {
    fn into_inner(&self) -> TransactionInput {
        self.inner.clone()
    }
}
