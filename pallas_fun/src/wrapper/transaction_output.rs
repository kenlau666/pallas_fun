use std::str::FromStr;

use hex;
use pallas::codec::utils::{Bytes, CborWrap};
use pallas::ledger::primitives::Fragment;
// use pallas::ledger::primitives::alonzo::TransactionOutput as LegacyTransactionOutput;
use pallas::ledger::primitives::babbage::PseudoPostAlonzoTransactionOutput;
use pallas::ledger::primitives::conway::{PseudoTransactionOutput, TransactionOutput};
use pallas::{
    codec::minicbor::{self, Decode, Encode},
    // ledger::primitives::conway::{DatumOption, ScriptRef},
};

use crate::utils::IntoInner;
use crate::wrapper::datum_option::DatumOptionWrapper;
use crate::wrapper::script_ref::ScriptRefWrapper;
use crate::wrapper::value::ValueWrapper;

// NOTE: implement this if legacy transaction output is needed
// pub enum TransactionOutputKind {
//     Legacy {
//         address: String,
//         amount: ValueWrapper,
//         datum_hash: Option<String>,
//     },
//     PostAlonzo {
//         address: String,
//         value: ValueWrapper,
//         datum_option: Option<DatumOptionWrapper>,
//         script_ref: Option<ScriptRefWrapper>,
//     },
// }

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)] // removed `Serialize` and `Deserialize`
pub struct TransactionOutputWrapper {
    #[n(0)]
    inner: TransactionOutput,
}

impl TransactionOutputWrapper {
    pub fn new(
        address: String,
        value: ValueWrapper,
        datum_option: Option<DatumOptionWrapper>,
        script_ref: Option<ScriptRefWrapper>,
    ) -> Result<Self, String> {
        let address =
            Bytes::from_str(&address).map_err(|e| format!("Invalid address bytes: {}", e))?;

        let pallas_transaction_output =
            PseudoTransactionOutput::PostAlonzo(PseudoPostAlonzoTransactionOutput {
                address,
                value: value.into_inner(),
                datum_option: datum_option.map(|w| w.into_inner()),
                script_ref: script_ref.map(|w| CborWrap(w.into_inner())),
            });

        Ok(Self {
            inner: pallas_transaction_output,
        })
    }
    // pub fn new(transaction_output_kind: TransactionOutputKind) -> Result<Self, String> {
    //     let pallas_transaction_output = match transaction_output_kind {
    //         TransactionOutputKind::Legacy {
    //             address,
    //             amount,
    //             datum_hash,
    //         } => PseudoTransactionOutput::Legacy(LegacyTransactionOutput {
    //             address: Bytes::from_str(&address)
    //                 .map_err(|e| format!("Invalid address bytes: {}", e))?,
    //             amount: amount.into_inner(),
    //             datum_hash: match datum_hash {
    //                 Some(hash) => Some(parse_datum_hash(&hash)?),
    //                 None => None,
    //             },
    //         }),
    //         TransactionOutputKind::PostAlonzo {
    //             address,
    //             value,
    //             datum_option,
    //             script_ref,
    //         } => PseudoTransactionOutput::PostAlonzo(PseudoPostAlonzoTransactionOutput {
    //             address: Bytes::from_str(&address)
    //                 .map_err(|e| format!("Invalid address bytes: {}", e))?,
    //             value: value.into_inner(),
    //             datum_option: datum_option.map(|w| w.into_inner()),
    //             script_ref: script_ref.map(|w| CborWrap(w.into_inner())),
    //         }),
    //     };

    //     Ok(Self {
    //         inner: pallas_transaction_output,
    //     })
    // }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let tx_input = TransactionOutput::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self { inner: tx_input })
    }
}

impl IntoInner<TransactionOutput> for TransactionOutputWrapper {
    fn into_inner(&self) -> TransactionOutput {
        self.inner.clone()
    }
}
