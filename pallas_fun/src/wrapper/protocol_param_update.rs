use hex;

use pallas::ledger::primitives::conway::{
    DRepVotingThresholds, ExUnitPrices, PoolVotingThresholds, ProtocolParamUpdate,
};
use pallas::ledger::primitives::{ExUnits, Fragment};
use pallas::{
    codec::minicbor::{self, Decode, Encode},
    ledger::primitives::conway::CostModels,
};
use serde::{Deserialize, Serialize};

use crate::utils::{IntoInner, parse_rational_number};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CostModelsWrapper {
    pub plutus_v1: Option<Vec<i64>>,
    pub plutus_v2: Option<Vec<i64>>,
    pub plutus_v3: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PoolVotingThresholdsWrapper {
    pub motion_no_confidence: (u64, u64),
    pub committee_normal: (u64, u64),
    pub committee_no_confidence: (u64, u64),
    pub hard_fork_initiation: (u64, u64),
    pub security_voting_threshold: (u64, u64),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DRepVotingThresholdsWrapper {
    pub motion_no_confidence: (u64, u64),
    pub committee_normal: (u64, u64),
    pub committee_no_confidence: (u64, u64),
    pub update_constitution: (u64, u64),
    pub hard_fork_initiation: (u64, u64),
    pub pp_network_group: (u64, u64),
    pub pp_economic_group: (u64, u64),
    pub pp_technical_group: (u64, u64),
    pub pp_governance_group: (u64, u64),
    pub treasury_withdrawal: (u64, u64),
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct ProtocolParamUpdateWrapper {
    #[n(0)]
    inner: ProtocolParamUpdate,
}

impl ProtocolParamUpdateWrapper {
    pub fn new(
        minfee_a: Option<u64>,
        minfee_b: Option<u64>,
        max_block_body_size: Option<u64>,
        max_transaction_size: Option<u64>,
        max_block_header_size: Option<u64>,
        key_deposit: Option<u64>,
        pool_deposit: Option<u64>,
        maximum_epoch: Option<u64>,
        desired_number_of_stake_pools: Option<u64>,
        pool_pledge_influence: Option<(u64, u64)>, // RationalNumber
        expansion_rate: Option<(u64, u64)>,
        treasury_growth_rate: Option<(u64, u64)>,
        min_pool_cost: Option<u64>,
        ada_per_utxo_byte: Option<u64>,
        cost_models_for_script_languages: Option<CostModelsWrapper>,
        execution_costs_mem_price: Option<(u64, u64)>,
        execution_costs_step_price: Option<(u64, u64)>, // ExUnitPrices
        max_tx_ex_mem: Option<u64>,
        max_tx_ex_steps: Option<u64>,
        max_block_ex_mem: Option<u64>,
        max_block_ex_steps: Option<u64>,
        max_value_size: Option<u64>,
        collateral_percentage: Option<u64>,
        max_collateral_inputs: Option<u64>,
        pool_voting_thresholds: Option<PoolVotingThresholdsWrapper>,
        drep_voting_thresholds: Option<DRepVotingThresholdsWrapper>,
        min_committee_size: Option<u64>,
        committee_term_limit: Option<u64>,
        governance_action_validity_period: Option<u64>,
        governance_action_deposit: Option<u64>,
        drep_deposit: Option<u64>,
        drep_inactivity_period: Option<u64>,
        minfee_refscript_cost_per_byte: Option<(u64, u64)>, // unit interval
    ) -> Result<Self, String> {
        let pool_pledge_influence = match pool_pledge_influence {
            Some((num, denom)) => Some(parse_rational_number(num, denom)?),
            None => None,
        };
        let expansion_rate = match expansion_rate {
            Some((num, denom)) => Some(parse_rational_number(num, denom)?),
            None => None,
        };
        let treasury_growth_rate = match treasury_growth_rate {
            Some((num, denom)) => Some(parse_rational_number(num, denom)?),
            None => None,
        };

        let cost_models_for_script_languages =
            cost_models_for_script_languages.map(|cm| CostModels {
                plutus_v1: cm.plutus_v1,
                plutus_v2: cm.plutus_v2,
                plutus_v3: cm.plutus_v3,
            });

        let execution_costs = match (execution_costs_mem_price, execution_costs_step_price) {
            (Some((mem_num, mem_denom)), Some((step_num, step_denom))) => Some(ExUnitPrices {
                mem_price: parse_rational_number(mem_num, mem_denom)?,
                step_price: parse_rational_number(step_num, step_denom)?,
            }),
            _ => None,
        };

        let max_tx_ex_units = match (max_tx_ex_mem, max_tx_ex_steps) {
            (Some(max_tx_ex_mem), Some(max_tx_ex_steps)) => Some(ExUnits {
                mem: max_tx_ex_mem,
                steps: max_tx_ex_steps,
            }),
            _ => None,
        };

        let max_block_ex_units = match (max_block_ex_mem, max_block_ex_steps) {
            (Some(max_block_ex_mem), Some(max_block_ex_steps)) => Some(ExUnits {
                mem: max_block_ex_mem,
                steps: max_block_ex_steps,
            }),
            _ => None,
        };

        let pool_voting_thresholds = match pool_voting_thresholds {
            Some(pool_voting_thresholds) => Some(PoolVotingThresholds {
                motion_no_confidence: parse_rational_number(
                    pool_voting_thresholds.motion_no_confidence.0,
                    pool_voting_thresholds.motion_no_confidence.1,
                )?,
                committee_normal: parse_rational_number(
                    pool_voting_thresholds.committee_normal.0,
                    pool_voting_thresholds.committee_normal.1,
                )?,
                committee_no_confidence: parse_rational_number(
                    pool_voting_thresholds.committee_no_confidence.0,
                    pool_voting_thresholds.committee_no_confidence.1,
                )?,
                hard_fork_initiation: parse_rational_number(
                    pool_voting_thresholds.hard_fork_initiation.0,
                    pool_voting_thresholds.hard_fork_initiation.1,
                )?,
                security_voting_threshold: parse_rational_number(
                    pool_voting_thresholds.security_voting_threshold.0,
                    pool_voting_thresholds.security_voting_threshold.1,
                )?,
            }),
            None => None,
        };

        let drep_voting_thresholds = match drep_voting_thresholds {
            Some(drep_voting_thresholds) => Some(DRepVotingThresholds {
                motion_no_confidence: parse_rational_number(
                    drep_voting_thresholds.motion_no_confidence.0,
                    drep_voting_thresholds.motion_no_confidence.1,
                )?,
                committee_normal: parse_rational_number(
                    drep_voting_thresholds.committee_normal.0,
                    drep_voting_thresholds.committee_normal.1,
                )?,
                committee_no_confidence: parse_rational_number(
                    drep_voting_thresholds.committee_no_confidence.0,
                    drep_voting_thresholds.committee_no_confidence.1,
                )?,
                update_constitution: parse_rational_number(
                    drep_voting_thresholds.update_constitution.0,
                    drep_voting_thresholds.update_constitution.1,
                )?,
                hard_fork_initiation: parse_rational_number(
                    drep_voting_thresholds.hard_fork_initiation.0,
                    drep_voting_thresholds.hard_fork_initiation.1,
                )?,
                pp_network_group: parse_rational_number(
                    drep_voting_thresholds.pp_network_group.0,
                    drep_voting_thresholds.pp_network_group.1,
                )?,
                pp_economic_group: parse_rational_number(
                    drep_voting_thresholds.pp_economic_group.0,
                    drep_voting_thresholds.pp_economic_group.1,
                )?,
                pp_technical_group: parse_rational_number(
                    drep_voting_thresholds.pp_technical_group.0,
                    drep_voting_thresholds.pp_technical_group.1,
                )?,
                pp_governance_group: parse_rational_number(
                    drep_voting_thresholds.pp_governance_group.0,
                    drep_voting_thresholds.pp_governance_group.1,
                )?,
                treasury_withdrawal: parse_rational_number(
                    drep_voting_thresholds.treasury_withdrawal.0,
                    drep_voting_thresholds.treasury_withdrawal.1,
                )?,
            }),
            None => None,
        };

        let minfee_refscript_cost_per_byte = match minfee_refscript_cost_per_byte {
            Some((num, denom)) => Some(parse_rational_number(num, denom)?),
            None => None,
        };

        Ok(Self {
            inner: ProtocolParamUpdate {
                minfee_a,
                minfee_b,
                max_block_body_size,
                max_transaction_size,
                max_block_header_size,
                key_deposit,
                pool_deposit,
                maximum_epoch,
                desired_number_of_stake_pools,
                pool_pledge_influence,
                expansion_rate,
                treasury_growth_rate,
                min_pool_cost,
                ada_per_utxo_byte,
                cost_models_for_script_languages,
                execution_costs,
                max_tx_ex_units,
                max_block_ex_units,
                max_value_size,
                collateral_percentage,
                max_collateral_inputs,
                pool_voting_thresholds,
                drep_voting_thresholds,
                min_committee_size,
                committee_term_limit,
                governance_action_validity_period,
                governance_action_deposit,
                drep_deposit,
                drep_inactivity_period,
                minfee_refscript_cost_per_byte,
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_protocol_param_update = ProtocolParamUpdate::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_protocol_param_update,
        })
    }
}

impl IntoInner<ProtocolParamUpdate> for ProtocolParamUpdateWrapper {
    fn into_inner(&self) -> ProtocolParamUpdate {
        self.inner.clone()
    }
}
