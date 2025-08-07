use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::{Fragment, StakeCredential};
use serde::{Deserialize, Serialize};

use crate::utils::{IntoInner, parse_address_key_hash, parse_script_hash};

pub enum StakeCredentialKind {
    ScriptHash(String),
    AddrKeyhash(String),
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct StakeCredentialWrapper {
    #[n(0)]
    pub pallas_stake_credential: StakeCredential,
}

impl StakeCredentialWrapper {
    pub fn new(stake_credential: StakeCredentialKind) -> Result<Self, String> {
        let pallas_stake_credential = match stake_credential {
            StakeCredentialKind::AddrKeyhash(keyhash) => {
                StakeCredential::AddrKeyhash(parse_address_key_hash(&keyhash)?)
            }
            StakeCredentialKind::ScriptHash(script_hash) => {
                StakeCredential::ScriptHash(parse_script_hash(&script_hash)?)
            }
        };

        Ok(StakeCredentialWrapper {
            pallas_stake_credential,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let stake_credential = StakeCredential::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self {
            pallas_stake_credential: stake_credential,
        })
    }
}

impl IntoInner<StakeCredential> for StakeCredentialWrapper {
    fn into_inner(&self) -> StakeCredential {
        self.pallas_stake_credential.clone()
    }
}
