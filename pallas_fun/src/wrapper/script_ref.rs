use std::str::FromStr;

use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::Bytes;
use pallas::ledger::primitives::conway::{NativeScript, ScriptRef};
use pallas::ledger::primitives::{Fragment, PlutusScript};

use crate::utils::IntoInner;

#[derive(Debug, Clone)]
pub enum ScriptRefKind {
    NativeScript { native_script_hex: String }, // NOTE: this requires a native script type instead of wrapper. Could wrap native script if needed.
    PlutusV1Script { plutus_v1_script: String },
    PlutusV2Script { plutus_v2_script: String },
    PlutusV3Script { plutus_v3_script: String },
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)] // removed `Serialize` and `Deserialize`
pub struct ScriptRefWrapper {
    #[n(0)]
    inner: ScriptRef,
}

impl ScriptRefWrapper {
    pub fn new(script_ref_kind: ScriptRefKind) -> Result<Self, String> {
        let pallas_script_ref = match script_ref_kind {
            ScriptRefKind::NativeScript { native_script_hex } => {
                let bytes = hex::decode(native_script_hex)
                    .map_err(|e| format!("Hex decode error: {}", e))?;
                let pallas_native_script = NativeScript::decode_fragment(&bytes)
                    .map_err(|e| format!("Fragment decode error: {}", e))?;
                ScriptRef::NativeScript(pallas_native_script)
            }

            ScriptRefKind::PlutusV1Script { plutus_v1_script } => {
                let bytes = Bytes::from_str(&plutus_v1_script)
                    .map_err(|e| format!("Invalid Plutus V1 script bytes: {}", e))?;
                ScriptRef::PlutusV1Script(PlutusScript::<1>(bytes))
            }

            ScriptRefKind::PlutusV2Script { plutus_v2_script } => {
                let bytes = Bytes::from_str(&plutus_v2_script)
                    .map_err(|e| format!("Invalid Plutus V2 script bytes: {}", e))?;
                ScriptRef::PlutusV2Script(PlutusScript::<2>(bytes))
            }

            ScriptRefKind::PlutusV3Script { plutus_v3_script } => {
                let bytes = Bytes::from_str(&plutus_v3_script)
                    .map_err(|e| format!("Invalid Plutus V3 script bytes: {}", e))?;
                ScriptRef::PlutusV3Script(PlutusScript::<3>(bytes))
            }
        };

        Ok(Self {
            inner: pallas_script_ref,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_script_ref = ScriptRef::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_script_ref,
        })
    }
}

impl IntoInner<ScriptRef> for ScriptRefWrapper {
    fn into_inner(&self) -> ScriptRef {
        self.inner.clone()
    }
}
