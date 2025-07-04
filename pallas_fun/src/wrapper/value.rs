use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::Value;
use serde::{Deserialize, Serialize};

use crate::wrapper::MultiassetPositiveCoinWrapper;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct ValueWrapper {
    #[n(0)]
    pub pallas_value: Value,
}

impl ValueWrapper {
    pub fn new(coin: u64, multiasset_wrapper: Option<MultiassetPositiveCoinWrapper>) -> Self {
        match multiasset_wrapper {
            Some(multiasset_wrapper) => Self {
                pallas_value: Value::Multiasset(coin, multiasset_wrapper.into_inner()),
            },
            None => Self {
                pallas_value: Value::Coin(coin),
            },
        }
    }

    pub fn encode(self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(self, hex_string: String) -> Self {
        Self {
            pallas_value: Value::decode_fragment(&hex::decode(hex_string).unwrap()).unwrap(),
        }
    }

    pub fn into_inner(self) -> Value {
        self.pallas_value
    }
}
