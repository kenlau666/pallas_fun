use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::PoolMetadata;
use serde::{Deserialize, Serialize};

use crate::utils::{IntoInner, parse_pool_metadata_hash};

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct PoolMetadataWrapper {
    #[n(0)]
    inner: PoolMetadata,
}

impl PoolMetadataWrapper {
    pub fn new(url: String, hash: String) -> Result<Self, String> {
        let hash = parse_pool_metadata_hash(&hash)?;

        Ok(Self {
            inner: PoolMetadata { url, hash },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_pool_metadata = PoolMetadata::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_pool_metadata,
        })
    }
}

impl IntoInner<PoolMetadata> for PoolMetadataWrapper {
    fn into_inner(&self) -> PoolMetadata {
        self.inner.clone()
    }
}
