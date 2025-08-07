use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::Bytes;
use pallas::ledger::primitives::{Fragment, RewardAccount};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
pub struct RewardAccountWrapper {
    #[n(0)]
     inner: RewardAccount,
}

impl RewardAccountWrapper {
    pub fn new(reward_account: &str) -> Result<Self, String> {
        let bytes = Bytes::from_str(reward_account)
            .map_err(|_| "Invalid reward account length".to_string())?;
        Ok(Self { inner: bytes })
    }

    pub fn encode(&self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let reward_account = RewardAccount::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self {
            inner: reward_account,
        })
    }
}

impl IntoInner<RewardAccount> for RewardAccountWrapper {
    fn into_inner(&self) -> RewardAccount {
        self.inner.clone()
    }
}
