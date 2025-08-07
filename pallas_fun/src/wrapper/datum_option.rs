use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::CborWrap;
use pallas::ledger::primitives::conway::DatumOption;
use pallas::ledger::primitives::{Fragment, PlutusData};

use crate::utils::{IntoInner, parse_datum_hash};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DatumOptionKind {
    Hash { datum_hash: String },
    Data { plutus_data_hex: String }, // Data(CborWrap<PlutusData>),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)] // removed `Serialize` and `Deserialize`
pub struct DatumOptionWrapper {
    #[n(0)]
    inner: DatumOption,
}

impl DatumOptionWrapper {
    pub fn new(datum_option_kind: DatumOptionKind) -> Result<Self, String> {
        let pallas_datum_option = match datum_option_kind {
            DatumOptionKind::Hash { datum_hash } => {
                let datum_hash = parse_datum_hash(&datum_hash)?;
                DatumOption::Hash(datum_hash)
            }

            DatumOptionKind::Data { plutus_data_hex } => {
                let bytes =
                    hex::decode(plutus_data_hex).map_err(|e| format!("Hex decode error: {}", e))?;
                PlutusData::decode_fragment(&bytes)
                    .map(|e| DatumOption::Data(CborWrap(e)))
                    .map_err(|e| format!("Fragment decode error: {}", e))?
            }
        };

        Ok(Self {
            inner: pallas_datum_option,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_datum_option = DatumOption::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_datum_option,
        })
    }
}

impl IntoInner<DatumOption> for DatumOptionWrapper {
    fn into_inner(&self) -> DatumOption {
        self.inner.clone()
    }
}
