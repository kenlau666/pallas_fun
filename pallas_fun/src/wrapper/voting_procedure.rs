use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::Nullable;
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::VotingProcedure;
use serde::{Deserialize, Serialize};

use crate::utils::IntoInner;
use crate::wrapper::anchor::AnchorWrapper;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum VoteKind {
    Yes,
    No,
    Abstain,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct VotingProcedureWrapper {
    #[n(0)]
    inner: VotingProcedure,
}

impl VotingProcedureWrapper {
    pub fn new(vote: VoteKind, anchor_wrapper: Option<AnchorWrapper>) -> Result<Self, String> {
        let vote = match vote {
            VoteKind::Yes => pallas::ledger::primitives::conway::Vote::Yes,
            VoteKind::No => pallas::ledger::primitives::conway::Vote::No,
            VoteKind::Abstain => pallas::ledger::primitives::conway::Vote::Abstain,
        };

        let anchor = anchor_wrapper
            .map(|aw| Nullable::from(Some(aw.into_inner())))
            .unwrap_or_else(|| Nullable::from(None));

        Ok(Self {
            inner: VotingProcedure { vote, anchor },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_voting_procedure = VotingProcedure::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_voting_procedure,
        })
    }
}

impl IntoInner<VotingProcedure> for VotingProcedureWrapper {
    fn into_inner(&self) -> VotingProcedure {
        self.inner.clone()
    }
}
