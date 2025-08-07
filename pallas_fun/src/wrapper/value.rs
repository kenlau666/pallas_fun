use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::Value;
use serde::{Deserialize, Serialize};

use crate::utils::IntoInner;
use crate::wrapper::MultiassetPositiveCoinWrapper;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct ValueWrapper {
    #[n(0)]
    inner: Value,
}

impl ValueWrapper {
    pub fn new(coin: u64, multiasset_wrapper: Option<MultiassetPositiveCoinWrapper>) -> Self {
        match multiasset_wrapper {
            Some(multiasset_wrapper) => Self {
                inner: Value::Multiasset(coin, multiasset_wrapper.into_inner()),
            },
            None => Self {
                inner: Value::Coin(coin),
            },
        }
    }

    pub fn encode(&self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let value =
            Value::decode_fragment(&bytes).map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self { inner: value })
    }
}

impl IntoInner<Value> for ValueWrapper {
    fn into_inner(&self) -> Value {
        self.inner.clone()
    }
}
