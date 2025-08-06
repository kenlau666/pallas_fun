use crate::utils::{IntoInner, parse_address_key_hash, parse_script_hash};
use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::DRep;
use serde::{Deserialize, Serialize};

pub enum DRepKind {
    Key { addr_key_hash: String },
    Script { script_hash: String },
    Abstain,
    NoConfidence,
}
#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct DRepWrapper {
    #[n(0)]
    inner: DRep,
}

impl DRepWrapper {
    pub fn new(drep: DRepKind) -> Result<Self, String> {
        let pallas_drep = match drep {
            DRepKind::Key { addr_key_hash } => DRep::Key(parse_address_key_hash(&addr_key_hash)?),
            DRepKind::Script { script_hash } => DRep::Script(parse_script_hash(&script_hash)?),
            DRepKind::Abstain => DRep::Abstain,
            DRepKind::NoConfidence => DRep::NoConfidence,
        };

        Ok(Self { inner: pallas_drep })
    }

    pub fn encode(&self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_drep =
            DRep::decode_fragment(&bytes).map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self { inner: pallas_drep })
    }
}

impl IntoInner<DRep> for DRepWrapper {
    fn into_inner(&self) -> DRep {
        self.inner.clone()
    }
}
