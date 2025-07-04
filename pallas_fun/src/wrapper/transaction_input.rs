pub use pallas::crypto::hash::Hash;

use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::{Fragment, TransactionInput};
use serde::{Deserialize, Serialize};

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
    pub pallas_transaction_input: TransactionInput,
}

impl TransactionInputWrapper {
    pub fn new(transaction_id: &str, index: u64) -> Self {
        let digest: Hash<32> = transaction_id
            .parse()
            .expect("Invalid transaction id length");

        Self {
            pallas_transaction_input: TransactionInput {
                transaction_id: digest,
                index: index,
            },
        }
    }

    pub fn encode(self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(self, hex_string: String) -> Self {
        Self {
            pallas_transaction_input: TransactionInput::decode_fragment(
                &hex::decode(hex_string).unwrap(),
            )
            .unwrap(),
        }
    }

    pub fn into_inner(self) -> TransactionInput {
        self.pallas_transaction_input
    }
}
