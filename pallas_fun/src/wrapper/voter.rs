use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::Voter;
use serde::{Deserialize, Serialize};

use crate::utils::{IntoInner, parse_address_key_hash, parse_script_hash};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum VoterKind {
    ConstitutionalCommitteeScript { script_hash: String },
    ConstitutionalCommitteeKey { script_hash: String },
    DRepScript { script_hash: String },
    DRepKey { addr_key_hash: String },
    StakePoolKey { addr_key_hash: String },
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct VoterWrapper {
    #[n(0)]
    pub pallas_voter: Voter,
}

impl VoterWrapper {
    pub fn new(voter: VoterKind) -> Result<Self, String> {
        let pallas_voter = match voter {
            VoterKind::ConstitutionalCommitteeKey { script_hash } => {
                Voter::ConstitutionalCommitteeKey(parse_script_hash(&script_hash)?)
            }
            VoterKind::ConstitutionalCommitteeScript { script_hash } => {
                Voter::ConstitutionalCommitteeScript(parse_script_hash(&script_hash)?)
            }
            VoterKind::DRepKey { addr_key_hash } => {
                Voter::DRepKey(parse_address_key_hash(&addr_key_hash)?)
            }
            VoterKind::DRepScript { script_hash } => {
                Voter::DRepScript(parse_script_hash(&script_hash)?)
            }
            VoterKind::StakePoolKey { addr_key_hash } => {
                Voter::StakePoolKey(parse_address_key_hash(&addr_key_hash)?)
            }
        };

        Ok(Self { pallas_voter })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let voter =
            Voter::decode_fragment(&bytes).map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self {
            pallas_voter: voter,
        })
    }
}

impl IntoInner<Voter> for VoterWrapper {
    fn into_inner(&self) -> Voter {
        self.pallas_voter.clone()
    }
}
