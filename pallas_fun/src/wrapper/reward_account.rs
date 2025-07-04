use std::str::FromStr;

use pallas::codec::utils::Bytes;

use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::{Fragment, RewardAccount};
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
pub struct RewardAccountWrapper {
    #[n(0)]
    pub pallas_reward_account: RewardAccount,
}

impl RewardAccountWrapper {
    pub fn new(reward_account: &str) -> Self {
        Self {
            pallas_reward_account: Bytes::from_str(reward_account)
                .expect("Invalid reward account length"),
        }
    }

    pub fn encode(self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(self, hex_string: String) -> Self {
        Self {
            pallas_reward_account: RewardAccount::decode_fragment(
                &hex::decode(hex_string).unwrap(),
            )
            .unwrap(),
        }
    }

    pub fn into_inner(self) -> RewardAccount {
        self.pallas_reward_account
    }
}
