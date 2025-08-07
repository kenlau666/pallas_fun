use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::{KeyValuePairs, Nullable};
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::GovAction;
use serde::{Deserialize, Serialize};

use crate::utils::{IntoInner, parse_rational_number, parse_script_hash, parse_vec_wrapper_to_set};
use crate::wrapper::constitution::ConstitutionWrapper;
use crate::wrapper::gov_action_id::GovActionIdWrapper;
use crate::wrapper::protocol_param_update::ProtocolParamUpdateWrapper;
use crate::wrapper::reward_account::RewardAccountWrapper;
use crate::wrapper::stake_credential::StakeCredentialWrapper;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GovActionKind {
    ParameterChange {
        gov_action_id_wrapper: Option<GovActionIdWrapper>,
        params: Box<ProtocolParamUpdateWrapper>,
        script_hash: Option<String>,
    },
    HardForkInitiation {
        gov_action_id_wrapper: Option<GovActionIdWrapper>,
        protocol_version: (u64, u64),
    },
    TreasuryWithdrawals {
        withdrawals: Vec<(RewardAccountWrapper, u64)>,
        script_hash: Option<String>,
    },
    NoConfidence {
        gov_action_id_wrapper: Option<GovActionIdWrapper>,
    },
    UpdateCommittee {
        gov_action_id_wrapper: Option<GovActionIdWrapper>,
        cold_credentials: Vec<StakeCredentialWrapper>,
        hot_credential: Vec<(StakeCredentialWrapper, u64)>,
        unit_interval: (u64, u64), // unit interval (nominator, denominator)
    },
    NewConstitution {
        gov_action_id_wrapper: Option<GovActionIdWrapper>,
        constitution_wrapper: ConstitutionWrapper,
    },
    Information,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct GovActionWrapper {
    #[n(0)]
    inner: GovAction,
}

impl GovActionWrapper {
    pub fn new(gov_action_kind: GovActionKind) -> Result<Self, String> {
        let pallas_gov_action = match gov_action_kind {
            GovActionKind::ParameterChange {
                gov_action_id_wrapper,
                params,
                script_hash,
            } => GovAction::ParameterChange(
                Nullable::from(gov_action_id_wrapper.map(|w| w.into_inner())),
                Box::new(params.into_inner()),
                Nullable::from(match script_hash {
                    Some(hash) => Some(parse_script_hash(&hash)?),
                    None => None,
                }),
            ),
            GovActionKind::HardForkInitiation {
                gov_action_id_wrapper,
                protocol_version,
            } => GovAction::HardForkInitiation(
                Nullable::from(gov_action_id_wrapper.map(|w| w.into_inner())),
                protocol_version,
            ),
            GovActionKind::TreasuryWithdrawals {
                withdrawals,
                script_hash,
            } => GovAction::TreasuryWithdrawals(
                KeyValuePairs::from_iter(withdrawals.into_iter().map(
                    |(reward_account_wrapper, amount)| {
                        (reward_account_wrapper.into_inner(), amount)
                    },
                )),
                Nullable::from(match script_hash {
                    Some(hash) => Some(parse_script_hash(&hash)?),
                    None => None,
                }),
            ),

            GovActionKind::NoConfidence {
                gov_action_id_wrapper,
            } => GovAction::NoConfidence(Nullable::from(
                gov_action_id_wrapper.map(|w| w.into_inner()),
            )),

            GovActionKind::UpdateCommittee {
                gov_action_id_wrapper,
                cold_credentials,
                hot_credential,
                unit_interval,
            } => GovAction::UpdateCommittee(
                Nullable::from(gov_action_id_wrapper.map(|w| w.into_inner())),
                parse_vec_wrapper_to_set(cold_credentials),
                KeyValuePairs::from_iter(
                    hot_credential
                        .into_iter()
                        .map(|(w, amount)| (w.into_inner(), amount)),
                ),
                parse_rational_number(unit_interval.0, unit_interval.1)?,
            ),

            GovActionKind::NewConstitution {
                gov_action_id_wrapper,
                constitution_wrapper,
            } => GovAction::NewConstitution(
                Nullable::from(gov_action_id_wrapper.map(|w| w.into_inner())),
                constitution_wrapper.into_inner(),
            ),

            GovActionKind::Information => GovAction::Information,
        };

        Ok(Self {
            inner: pallas_gov_action,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let gov_action = GovAction::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self { inner: gov_action })
    }
}

impl IntoInner<GovAction> for GovActionWrapper {
    fn into_inner(&self) -> GovAction {
        self.inner.clone()
    }
}
