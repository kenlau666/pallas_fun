use pallas::codec::utils::NonEmptySet;

use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::ledger::primitives::conway::RequiredSigners;
use pallas::ledger::primitives::{AddrKeyhash, Fragment};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct RequiredSignersWrapper {
    #[n(0)]
    pub pallas_require_signers: RequiredSigners,
}

impl RequiredSignersWrapper {
    fn parse_address_key_hash(address_key_hash_str: &str) -> AddrKeyhash {
        address_key_hash_str
            .parse()
            .expect("Invalid address key hash length")
    }

    pub fn new(required_signers: Vec<&str>) -> Self {
        let pallas_require_signers_vec: Vec<AddrKeyhash> = required_signers
            .into_iter()
            .map(|address_key_hash_str| Self::parse_address_key_hash(address_key_hash_str))
            .collect();
        Self {
            pallas_require_signers: NonEmptySet::from_vec(pallas_require_signers_vec)
                .expect("invalid require signers"),
        }
    }

    pub fn encode(self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(self, hex_string: String) -> Self {
        Self {
            pallas_require_signers: RequiredSigners::decode_fragment(
                &hex::decode(hex_string).unwrap(),
            )
            .unwrap(),
        }
    }

    pub fn into_inner(self) -> RequiredSigners {
        self.pallas_require_signers
    }
}
