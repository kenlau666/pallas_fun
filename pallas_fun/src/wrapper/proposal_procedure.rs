use hex;
use pallas::ledger::primitives::Fragment;
use pallas::{
    codec::minicbor::{self, Decode, Encode},
    ledger::primitives::conway::ProposalProcedure,
};
use serde::{Deserialize, Serialize};

use crate::utils::IntoInner;
use crate::wrapper::anchor::AnchorWrapper;
use crate::wrapper::gov_action::GovActionWrapper;
use crate::wrapper::reward_account::RewardAccountWrapper;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct ProposalProcedureWrapper {
    #[n(0)]
    inner: ProposalProcedure,
}

impl ProposalProcedureWrapper {
    pub fn new(
        deposit: u64,
        reward_account_wrapper: RewardAccountWrapper,
        gov_action_wrapper: GovActionWrapper,
        anchor_wrapper: AnchorWrapper,
    ) -> Result<Self, String> {
        Ok(Self {
            inner: ProposalProcedure {
                deposit,
                reward_account: reward_account_wrapper.into_inner(),
                gov_action: gov_action_wrapper.into_inner(),
                anchor: anchor_wrapper.into_inner(),
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let tx_input = ProposalProcedure::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self { inner: tx_input })
    }
}

impl IntoInner<ProposalProcedure> for ProposalProcedureWrapper {
    fn into_inner(&self) -> ProposalProcedure {
        self.inner.clone()
    }
}
