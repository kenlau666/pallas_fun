use std::str::FromStr;

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
    fn parse_address_key_hash(address_key_hash_str: &str) -> AddrKeyhash {
        address_key_hash_str
            .parse()
            .expect("Invalid address key hash length")
    }

    fn parse_script_hash(script_hash_str: &str) -> ScriptHash {
        script_hash_str.parse().expect("Invalid script hash length")
    }

    pub fn new(voter: VoterKind) -> Self {
        let pallas_voter = match voter {
            VoterKind::ConstitutionalCommitteeKey(keyhash) => {
                Voter::ConstitutionalCommitteeKey(Self::parse_address_key_hash(&keyhash))
            }
            VoterKind::ConstitutionalCommitteeScript(script_hash) => {
                Voter::ConstitutionalCommitteeScript(Self::parse_script_hash(&script_hash))
            }
            VoterKind::DRepKey(keyhash) => Voter::DRepKey(Self::parse_address_key_hash(&keyhash)),
            VoterKind::DRepScript(script_hash) => {
                Voter::DRepScript(Self::parse_script_hash(&script_hash))
            }
            VoterKind::StakePoolKey(keyhash) => {
                Voter::StakePoolKey(Self::parse_address_key_hash(&keyhash))
            }
        };

        VoterWrapper { pallas_voter }
    }

    pub fn encode(self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(self, hex_string: String) -> Self {
        Self {
            pallas_voter: Voter::decode_fragment(&hex::decode(hex_string).unwrap()).unwrap(),
        }
    }

    pub fn into_inner(self) -> Voter {
        self.pallas_voter
    }
}
