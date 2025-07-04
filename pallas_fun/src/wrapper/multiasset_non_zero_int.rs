use std::str::FromStr;

use pallas::codec::utils::{Bytes, NonEmptyKeyValuePairs, NonZeroInt};

use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::conway::Multiasset;
use pallas::ledger::primitives::{AssetName, Fragment, PolicyId};
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
pub struct MultiassetNonZeroIntWrapper {
    #[n(0)]
    pub pallas_multiasset: Multiasset<NonZeroInt>,
}

impl MultiassetNonZeroIntWrapper {
    fn parse_policy_id(policy_str: &str) -> PolicyId {
        policy_str.parse().expect("Invalid policy ID length")
    }

    fn parse_asset_name(asset_str: &str) -> AssetName {
        Bytes::from_str(asset_str).expect("Invalid asset name length")
    }

    fn parse_asset_vec(
        asset_vec: Vec<(String, i64)>,
    ) -> NonEmptyKeyValuePairs<AssetName, NonZeroInt> {
        let asset_pallas = asset_vec
            .into_iter()
            .map(|(asset_str, amount_i64)| {
                let asset_name = Self::parse_asset_name(&asset_str);
                let amount = NonZeroInt::try_from(amount_i64).expect("Invalid amount");
                (asset_name, amount)
            })
            .collect();

        NonEmptyKeyValuePairs::from_vec(asset_pallas).expect("Invalid asset")
    }
    fn convert_to_pallas_multiasset(
        input: Vec<(String, Vec<(String, i64)>)>,
    ) -> Multiasset<NonZeroInt> {
        let mut pallas_multiasset_vec: Vec<(
            PolicyId,
            NonEmptyKeyValuePairs<AssetName, NonZeroInt>,
        )> = Vec::new();

        for (policy_str, asset_vec) in input {
            let policy_id = Self::parse_policy_id(&policy_str);
            let asset_pallas = Self::parse_asset_vec(asset_vec);

            pallas_multiasset_vec.push((policy_id, asset_pallas));
        }

        NonEmptyKeyValuePairs::from_vec(pallas_multiasset_vec).expect("Invalid multiasset")
    }

    pub fn new(multiasset: Vec<(String, Vec<(String, i64)>)>) -> Self {
        Self {
            pallas_multiasset: Self::convert_to_pallas_multiasset(multiasset),
        }
    }

    pub fn encode(self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(self, hex_string: String) -> Self {
        Self {
            pallas_multiasset: NonEmptyKeyValuePairs::decode_fragment(
                &hex::decode(hex_string).unwrap(),
            )
            .unwrap(),
        }
    }

    pub fn into_inner(self) -> Multiasset<NonZeroInt> {
        self.pallas_multiasset
    }
}
