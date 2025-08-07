use hex;
use pallas::codec::utils::Nullable;
use pallas::ledger::primitives::Fragment;
use pallas::{
    codec::minicbor::{self, Decode, Encode},
    ledger::primitives::conway::Constitution,
};
use serde::{Deserialize, Serialize};

use crate::utils::{IntoInner, parse_script_hash};
use crate::wrapper::anchor::AnchorWrapper;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct ConstitutionWrapper {
    #[n(0)]
    inner: Constitution,
}

impl ConstitutionWrapper {
    pub fn new(
        anchor_wrapper: AnchorWrapper,
        guardrail_script_hash: Option<String>,
    ) -> Result<Self, String> {
        let guardrail_script = match guardrail_script_hash {
            Some(hash) => Nullable::Some(parse_script_hash(&hash)?),
            None => Nullable::Null,
        };

        Ok(Self {
            inner: Constitution {
                anchor: anchor_wrapper.into_inner(),
                guardrail_script,
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_constitution = Constitution::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_constitution,
        })
    }
}

impl IntoInner<Constitution> for ConstitutionWrapper {
    fn into_inner(&self) -> Constitution {
        self.inner.clone()
    }
}
