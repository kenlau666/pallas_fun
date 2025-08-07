use std::str::FromStr;

use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::{
    Bytes, NonEmptyKeyValuePairs, NonEmptySet, NonZeroInt, PositiveCoin, Set,
};
use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::{
    Certificate, GovActionId, Multiasset, ProposalProcedure, RequiredSigners, TransactionBody,
    TransactionOutput, Voter, VotingProcedure,
};
use pallas::ledger::primitives::{Coin, Fragment, NetworkId, RewardAccount, TransactionInput};

use crate::utils::{IntoInner, parse_vec_wrapper_to_set};
use crate::wrapper::certificate::CertificateWrapper;
use crate::wrapper::gov_action_id::GovActionIdWrapper;
use crate::wrapper::multiasset_non_zero_int::MultiassetNonZeroIntWrapper;
use crate::wrapper::proposal_procedure::ProposalProcedureWrapper;
use crate::wrapper::required_signers::RequiredSignersWrapper;
use crate::wrapper::reward_account::RewardAccountWrapper;
use crate::wrapper::voter::VoterWrapper;
use crate::wrapper::voting_procedure::VotingProcedureWrapper;
use crate::wrapper::{TransactionInputWrapper, TransactionOutputWrapper};

#[derive(Encode, Decode, Debug, PartialEq, Clone)] // removed serialize and deserialize traits
#[cbor(map)]
pub struct TransactionBodyWrapper {
    #[n(0)]
    inner: TransactionBody,
}

impl TransactionBodyWrapper {
    pub fn new(
        inputs: Vec<TransactionInputWrapper>,
        outputs: Vec<TransactionOutputWrapper>,
        fee: u64,
        ttl: Option<u64>,
        certificates: Option<Vec<CertificateWrapper>>,
        withdrawals: Option<Vec<(RewardAccountWrapper, u64)>>, // Option<NonEmptyKeyValuePairs<RewardAccount, Coin>>
        auxiliary_data_hash: Option<String>,                   // Bytes
        validity_interval_start: Option<u64>,
        mint: Option<MultiassetNonZeroIntWrapper>,
        script_data_hash: Option<String>,                 // hash 32
        collateral: Option<Vec<TransactionInputWrapper>>, // Option<NonEmptySet<TransactionInput>>,
        required_signers: Option<RequiredSignersWrapper>,
        network_id: Option<NetworkId>,
        collateral_return: Option<TransactionOutputWrapper>,
        total_collateral: Option<u64>,
        reference_inputs: Option<Vec<TransactionInputWrapper>>, // Option<NonEmptySet<TransactionInput>>,
        voting_procedures: Option<
            Vec<(
                VoterWrapper,
                Vec<(GovActionIdWrapper, VotingProcedureWrapper)>,
            )>,
        >, // NonEmptyKeyValuePairs<Voter, NonEmptyKeyValuePairs<GovActionId, VotingProcedure>>;
        proposal_procedures: Option<Vec<ProposalProcedureWrapper>>, // Option<NonEmptySet<ProposalProcedure>>,
        treasury_value: Option<u64>,                                // coin
        donation: Option<u64>,
    ) -> Result<Self, String> {
        // TODO: error handling for parsing functions
        let inputs = Self::parse_inputs(inputs);
        let outputs = Self::parse_transaction_outputs(outputs);
        let certificates = Self::parse_certificates(certificates);
        let withdrawals = Self::parse_withdrawals(withdrawals);
        let auxiliary_data_hash = Self::parse_auxiliary_data_hash(auxiliary_data_hash)?;
        let mint = Self::parse_mint(mint);
        let script_data_hash = Self::parse_script_data_hash(script_data_hash)?;
        let collateral = Self::parse_collateral(collateral);
        let required_signers = Self::parse_required_signers(required_signers);
        let collateral_return = Self::parse_collateral_return(collateral_return);
        let reference_inputs = Self::parse_reference_inputs(reference_inputs);
        let voting_procedures = Self::parse_voting_procedures(voting_procedures);
        let proposal_procedures = Self::parse_proposal_procedures(proposal_procedures);
        let donation = Self::parse_donation(donation);

        Ok(Self {
            inner: TransactionBody {
                inputs,
                outputs,
                fee,
                ttl,
                certificates,
                withdrawals,
                auxiliary_data_hash,
                validity_interval_start,
                mint,
                script_data_hash,
                collateral,
                required_signers,
                network_id,
                collateral_return,
                total_collateral,
                reference_inputs,
                voting_procedures,
                proposal_procedures,
                treasury_value,
                donation,
            },
        })
    }

    // TODO: Implement error handling for parsing functions
    // NOTE: multiple parser convert option vec to option non empty set, could reduce to one function with different parameter inputs

    pub fn parse_inputs(inputs: Vec<TransactionInputWrapper>) -> Set<TransactionInput> {
        parse_vec_wrapper_to_set(inputs)
    }

    pub fn parse_transaction_outputs(
        outputs: Vec<TransactionOutputWrapper>,
    ) -> Vec<TransactionOutput> {
        outputs
            .into_iter()
            .map(|output| output.into_inner())
            .collect()
    }

    pub fn parse_collateral_return(
        collateral_return: Option<TransactionOutputWrapper>,
    ) -> Option<TransactionOutput> {
        collateral_return.map(|output| output.into_inner())
    }

    pub fn parse_script_data_hash(
        script_data_hash: Option<String>,
    ) -> Result<Option<Hash<32>>, String> {
        match script_data_hash {
            Some(hash) => Hash::<32>::from_str(&hash)
                .map(Some)
                .map_err(|_| "Invalid script data hash length".to_string()),
            None => Ok(None),
        }
    }

    pub fn parse_collateral(
        collateral: Option<Vec<TransactionInputWrapper>>,
    ) -> Option<NonEmptySet<TransactionInput>> {
        collateral.map(|c| {
            NonEmptySet::from_vec(c.into_iter().map(|w| w.into_inner()).collect()).unwrap()
        })
    }

    pub fn parse_proposal_procedures(
        proposal_procedures: Option<Vec<ProposalProcedureWrapper>>,
    ) -> Option<NonEmptySet<ProposalProcedure>> {
        proposal_procedures.map(|pp| {
            NonEmptySet::from_vec(pp.into_iter().map(|p| p.into_inner()).collect()).unwrap()
        })
    }

    pub fn parse_reference_inputs(
        reference_inputs: Option<Vec<TransactionInputWrapper>>,
    ) -> Option<NonEmptySet<TransactionInput>> {
        reference_inputs.map(|inputs| {
            NonEmptySet::from_vec(inputs.into_iter().map(|w| w.into_inner()).collect()).unwrap()
        })
    }

    pub fn parse_voting_procedures(
        voting_procedures: Option<
            Vec<(
                VoterWrapper,
                Vec<(GovActionIdWrapper, VotingProcedureWrapper)>,
            )>,
        >,
    ) -> Option<NonEmptyKeyValuePairs<Voter, NonEmptyKeyValuePairs<GovActionId, VotingProcedure>>>
    {
        voting_procedures.map(|vp| {
            NonEmptyKeyValuePairs::from_vec(
                vp.into_iter()
                    .map(|(voter_wrapper, procedures)| {
                        (
                            voter_wrapper.into_inner(),
                            NonEmptyKeyValuePairs::from_vec(
                                procedures
                                    .into_iter()
                                    .map(|(gov_action_id_wrapper, voting_procedure_wrapper)| {
                                        (
                                            gov_action_id_wrapper.into_inner(),
                                            voting_procedure_wrapper.into_inner(),
                                        )
                                    })
                                    .collect(),
                            )
                            .unwrap(),
                        )
                    })
                    .collect(),
            )
            .unwrap()
        })
    }

    pub fn parse_withdrawals(
        withdrawals: Option<Vec<(RewardAccountWrapper, u64)>>,
    ) -> Option<NonEmptyKeyValuePairs<RewardAccount, Coin>> {
        withdrawals.map(|w| {
            NonEmptyKeyValuePairs::from_vec(
                w.into_iter()
                    .map(|(ra, coin)| (ra.into_inner(), Coin::try_from(coin).unwrap()))
                    .collect(),
            )
            .unwrap()
        })
    }

    pub fn parse_auxiliary_data_hash(
        auxiliary_data_hash: Option<String>,
    ) -> Result<Option<Bytes>, String> {
        auxiliary_data_hash
            .map(|hash| Bytes::from_str(&hash).map_err(|e| e.to_string()))
            .transpose()
    }

    pub fn parse_certificates(
        certificates: Option<Vec<CertificateWrapper>>,
    ) -> Option<NonEmptySet<Certificate>> {
        certificates.map(|certs| {
            NonEmptySet::from_vec(certs.into_iter().map(|c| c.into_inner()).collect()).unwrap()
        })
    }

    pub fn parse_mint(mint: Option<MultiassetNonZeroIntWrapper>) -> Option<Multiasset<NonZeroInt>> {
        mint.map(|wrapper| wrapper.into_inner())
    }

    pub fn parse_required_signers(
        required_signers: Option<RequiredSignersWrapper>,
    ) -> Option<RequiredSigners> {
        required_signers.map(|wrapper| wrapper.into_inner())
    }

    pub fn parse_donation(donation: Option<u64>) -> Option<PositiveCoin> {
        donation.map(|d| PositiveCoin::try_from(d).ok()).flatten()
    }

    pub fn encode(&self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let transaction_body = TransactionBody::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self {
            inner: transaction_body,
        })
    }
}

impl IntoInner<TransactionBody> for TransactionBodyWrapper {
    fn into_inner(&self) -> TransactionBody {
        self.inner.clone()
    }
}

// #[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Clone)]
// #[cbor(map)]
// pub struct TransactionBody<'a> {
//     #[n(0)]
//     pub inputs: Set<TransactionInput>,

//     #[b(1)]
//     pub outputs: Vec<TransactionOutput<'a>>,

//     #[n(2)]
//     pub fee: u64, // Coin = u64

//     #[n(3)]
//     pub ttl: Option<u64>,

//     #[n(4)]
//     pub certificates: Option<NonEmptySet<Certificate>>,

//     #[n(5)]
//     pub withdrawals: Option<BTreeMap<RewardAccount, Coin>>,

//     #[n(7)]
//     pub auxiliary_data_hash: Option<Bytes>,

//     #[n(8)]
//     pub validity_interval_start: Option<u64>,

//     #[n(9)]
//     pub mint: Option<Multiasset<NonZeroInt>>,

//     #[n(11)]
//     pub script_data_hash: Option<Hash<32>>,

//     #[n(13)]
//     pub collateral: Option<NonEmptySet<TransactionInput>>,

//     #[n(14)]
//     pub required_signers: Option<RequiredSigners>,

//     #[n(15)]
//     pub network_id: Option<NetworkId>,

//     #[n(16)]
//     pub collateral_return: Option<TransactionOutput<'a>>,

//     #[n(17)]
//     pub total_collateral: Option<u64>, // Coin = u64

//     #[n(18)]
//     pub reference_inputs: Option<NonEmptySet<TransactionInput>>,

//     // -- NEW IN CONWAY
//     #[n(19)]
//     pub voting_procedures: Option<VotingProcedures>,

//     #[n(20)]
//     pub proposal_procedures: Option<NonEmptySet<ProposalProcedure>>,

//     #[n(21)]
//     pub treasury_value: Option<u64>, // Coin = u64

//     #[n(22)]
//     pub donation: Option<PositiveCoin>,
// }
