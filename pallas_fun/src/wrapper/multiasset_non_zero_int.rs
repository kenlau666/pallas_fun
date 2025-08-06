use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::{Bytes, NonEmptyKeyValuePairs, NonZeroInt};
use pallas::ledger::primitives::{AssetName, Fragment, PolicyId, conway::Multiasset};
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
pub struct MultiassetNonZeroIntWrapper {
    #[n(0)]
    inner: Multiasset<NonZeroInt>,
}

impl MultiassetNonZeroIntWrapper {
    fn parse_policy_id(policy_str: &str) -> Result<PolicyId, String> {
        policy_str
            .parse()
            .map_err(|_| "Invalid policy ID length".to_string())
    }

    fn parse_asset_name(asset_str: &str) -> Result<AssetName, String> {
        Bytes::from_str(asset_str).map_err(|_| "Invalid asset name length".to_string())
    }

    fn parse_asset_vec(
        asset_vec: Vec<(String, i64)>,
    ) -> Result<NonEmptyKeyValuePairs<AssetName, NonZeroInt>, String> {
        let mut asset_pallas = Vec::new();
        for (asset_str, amount_i64) in asset_vec {
            let asset_name = Self::parse_asset_name(&asset_str)?;
            let amount =
                NonZeroInt::try_from(amount_i64).map_err(|_| "Invalid amount".to_string())?;
            asset_pallas.push((asset_name, amount));
        }
        NonEmptyKeyValuePairs::from_vec(asset_pallas).ok_or_else(|| "Invalid asset".to_string())
    }

    fn convert_to_pallas_multiasset(
        input: Vec<(String, Vec<(String, i64)>)>,
    ) -> Result<Multiasset<NonZeroInt>, String> {
        let mut pallas_multiasset_vec: Vec<(
            PolicyId,
            NonEmptyKeyValuePairs<AssetName, NonZeroInt>,
        )> = Vec::new();

        for (policy_str, asset_vec) in input {
            let policy_id = Self::parse_policy_id(&policy_str)?;
            let asset_pallas = Self::parse_asset_vec(asset_vec)?;
            pallas_multiasset_vec.push((policy_id, asset_pallas));
        }

        NonEmptyKeyValuePairs::from_vec(pallas_multiasset_vec)
            .ok_or_else(|| "Invalid multiasset".to_string())
    }

    pub fn new(multiasset: Vec<(String, Vec<(String, i64)>)>) -> Result<Self, String> {
        let pallas_multiasset = Self::convert_to_pallas_multiasset(multiasset)?;
        Ok(Self {
            inner: pallas_multiasset,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let multiasset = NonEmptyKeyValuePairs::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self { inner: multiasset })
    }
}

impl IntoInner<Multiasset<NonZeroInt>> for MultiassetNonZeroIntWrapper {
    fn into_inner(&self) -> Multiasset<NonZeroInt> {
        self.inner.clone()
    }
}
