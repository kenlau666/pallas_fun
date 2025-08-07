use hex;
pub use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::Fragment;
use pallas::{
    codec::minicbor::{self, Decode, Encode},
    ledger::primitives::conway::Anchor,
};
use serde::{Deserialize, Serialize};

use crate::utils::IntoInner;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct AnchorWrapper {
    #[n(0)]
    inner: Anchor,
}

impl AnchorWrapper {
    pub fn new(url: String, content_hash: String) -> Result<Self, String> {
        let content_hash: Hash<32> = content_hash
            .parse()
            .map_err(|_| "Invalid transaction id length".to_string())?;

        Ok(Self {
            inner: Anchor { url, content_hash },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_anchor =
            Anchor::decode_fragment(&bytes).map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_anchor,
        })
    }
}

impl IntoInner<Anchor> for AnchorWrapper {
    fn into_inner(&self) -> Anchor {
        self.inner.clone()
    }
}
