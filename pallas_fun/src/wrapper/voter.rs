use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::conway::Voter;
use pallas::ledger::primitives::{AddrKeyhash, Fragment, ScriptHash};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum VoterKind {
    ConstitutionalCommitteeKey(String),
    ConstitutionalCommitteeScript(String),
    DRepKey(String),
    DRepScript(String),
    StakePoolKey(String),
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct VoterWrapper {
    #[n(0)]
    pub pallas_voter: Voter,
}

impl VoterWrapper {
    fn parse_address_key_hash(address_key_hash_str: &str) -> Result<AddrKeyhash, String> {
        address_key_hash_str
            .parse()
            .map_err(|_| "Invalid address key hash length".to_string())
    }

    fn parse_script_hash(script_hash_str: &str) -> Result<ScriptHash, String> {
        script_hash_str
            .parse()
            .map_err(|_| "Invalid script hash length".to_string())
    }

    pub fn new(voter: VoterKind) -> Result<Self, String> {
        let pallas_voter = match voter {
            VoterKind::ConstitutionalCommitteeKey(keyhash) => {
                Voter::ConstitutionalCommitteeKey(Self::parse_address_key_hash(&keyhash)?)
            }
            VoterKind::ConstitutionalCommitteeScript(script_hash) => {
                Voter::ConstitutionalCommitteeScript(Self::parse_script_hash(&script_hash)?)
            }
            VoterKind::DRepKey(keyhash) => Voter::DRepKey(Self::parse_address_key_hash(&keyhash)?),
            VoterKind::DRepScript(script_hash) => {
                Voter::DRepScript(Self::parse_script_hash(&script_hash)?)
            }
            VoterKind::StakePoolKey(keyhash) => {
                Voter::StakePoolKey(Self::parse_address_key_hash(&keyhash)?)
            }
        };

        Ok(VoterWrapper { pallas_voter })
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

    pub fn into_inner(&self) -> Voter {
        self.pallas_voter.clone()
    }
}
