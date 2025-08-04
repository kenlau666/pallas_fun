use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::NonEmptySet;
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::RequiredSigners;
use serde::{Deserialize, Serialize};

use crate::utils::{IntoInner, parse_address_key_hash};

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct RequiredSignersWrapper {
    #[n(0)]
    pub pallas_require_signers: RequiredSigners,
}

impl RequiredSignersWrapper {
    pub fn new(required_signers: Vec<&str>) -> Result<Self, String> {
        let mut pallas_require_signers_vec = Vec::new();
        for address_key_hash_str in required_signers {
            let keyhash = parse_address_key_hash(address_key_hash_str)?;
            pallas_require_signers_vec.push(keyhash);
        }
        let non_empty_set = NonEmptySet::from_vec(pallas_require_signers_vec)
            .ok_or_else(|| "invalid require signers".to_string())?;

        Ok(Self {
            pallas_require_signers: non_empty_set,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let required_signers = RequiredSigners::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self {
            pallas_require_signers: required_signers,
        })
    }
}

impl IntoInner<RequiredSigners> for RequiredSignersWrapper {
    fn into_inner(&self) -> RequiredSigners {
        self.pallas_require_signers.clone()
    }
}
