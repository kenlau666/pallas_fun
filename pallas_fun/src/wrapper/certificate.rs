use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::Nullable;
use pallas::ledger::primitives::Fragment;
use pallas::ledger::primitives::conway::Certificate;
use serde::{Deserialize, Serialize};

use crate::utils::{
    IntoInner, parse_pool_key_hash, parse_rational_number, parse_vec_string_to_set_addr_keyhash,
    parse_vrf_key_hash,
};
use crate::wrapper::anchor::AnchorWrapper;
use crate::wrapper::d_rep::DRepWrapper;
use crate::wrapper::pool_metadata::PoolMetadataWrapper;
use crate::wrapper::relay::RelayWrapper;
use crate::wrapper::reward_account::RewardAccountWrapper;
use crate::wrapper::stake_credential::StakeCredentialWrapper;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum CertificateKind {
    StakeRegistration {
        stake_credential_wrapper: StakeCredentialWrapper,
    },
    StakeDeregistration {
        stake_credential_wrapper: StakeCredentialWrapper,
    },
    StakeDelegation {
        stake_credential_wrapper: StakeCredentialWrapper,
        pool_key_hash: String,
    },

    PoolRegistration {
        operator: String,    // pool key hash
        vrf_keyhash: String, // vrf key hash
        pledge: u64,
        cost: u64,
        margin_nominator: u64,
        margin_denominator: u64,
        reward_account_wrapper: RewardAccountWrapper,
        pool_owners: Vec<String>, // set of pool owner addr key hashes
        relay_wrappers: Vec<RelayWrapper>,
        pool_metadata_wrapper: Option<PoolMetadataWrapper>, // Nullable PoolMetadata
    },
    PoolRetirement {
        pool_key_hash: String,
        epoch: u64,
    },

    Reg {
        stake_credential_wrapper: StakeCredentialWrapper,
        amount: u64,
    },
    UnReg {
        stake_credential_wrapper: StakeCredentialWrapper,
        amount: u64,
    },
    VoteDeleg {
        stake_credential_wrapper: StakeCredentialWrapper,
        drep_wrapper: DRepWrapper,
    },
    StakeVoteDeleg {
        stake_credential_wrapper: StakeCredentialWrapper,
        pool_key_hash: String,
        drep_wrapper: DRepWrapper,
    },
    StakeRegDeleg {
        stake_credential_wrapper: StakeCredentialWrapper,
        pool_key_hash: String,
        amount: u64,
    },
    VoteRegDeleg {
        stake_credential_wrapper: StakeCredentialWrapper,
        drep_wrapper: DRepWrapper,
        amount: u64,
    },
    StakeVoteRegDeleg {
        stake_credential_wrapper: StakeCredentialWrapper,
        pool_key_hash: String,
        drep_wrapper: DRepWrapper,
        amount: u64,
    },

    AuthCommitteeHot {
        committee_cold_cred: StakeCredentialWrapper,
        committee_hot_cred: StakeCredentialWrapper,
    },
    ResignCommitteeCold {
        committee_cold_cred: StakeCredentialWrapper,
        anchor_wrapper: Option<AnchorWrapper>,
    },
    RegDRepCert {
        drep_cred: StakeCredentialWrapper,
        amount: u64,
        anchor_wrapper: Option<AnchorWrapper>,
    },
    UnRegDRepCert {
        drep_cred: StakeCredentialWrapper,
        amount: u64,
    },
    UpdateDRepCert {
        drep_cred: StakeCredentialWrapper,
        anchor_wrapper: Option<AnchorWrapper>,
    },
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)] // removed partialOrd and ord
pub struct CertificateWrapper {
    #[n(0)]
    inner: Certificate,
}

impl CertificateWrapper {
    pub fn new(certificate: CertificateKind) -> Result<Self, String> {
        // Convert the CertificateKind into a Pallas Certificate
        let pallas_certificate = match certificate {
            CertificateKind::StakeRegistration {
                stake_credential_wrapper,
            } => Certificate::StakeRegistration(stake_credential_wrapper.into_inner()),

            CertificateKind::StakeDeregistration {
                stake_credential_wrapper,
            } => Certificate::StakeDeregistration(stake_credential_wrapper.into_inner()),

            CertificateKind::StakeDelegation {
                stake_credential_wrapper,
                pool_key_hash,
            } => Certificate::StakeDelegation(
                stake_credential_wrapper.into_inner(),
                parse_pool_key_hash(&pool_key_hash)?,
            ),

            CertificateKind::PoolRegistration {
                operator,
                vrf_keyhash,
                pledge,
                cost,
                margin_nominator,
                margin_denominator,
                reward_account_wrapper,
                pool_owners,
                relay_wrappers,
                pool_metadata_wrapper,
            } => Certificate::PoolRegistration {
                operator: parse_pool_key_hash(&operator)?,
                vrf_keyhash: parse_vrf_key_hash(&vrf_keyhash)?,
                pledge,
                cost,
                margin: parse_rational_number(margin_nominator, margin_denominator)?,
                reward_account: reward_account_wrapper.into_inner(),
                pool_owners: parse_vec_string_to_set_addr_keyhash(pool_owners)?,
                relays: relay_wrappers.into_iter().map(|r| r.into_inner()).collect(),
                pool_metadata: Nullable::from(pool_metadata_wrapper.map(|pm| pm.into_inner())),
            },

            CertificateKind::PoolRetirement {
                pool_key_hash,
                epoch,
            } => Certificate::PoolRetirement(parse_pool_key_hash(&pool_key_hash)?, epoch),

            CertificateKind::Reg {
                stake_credential_wrapper,
                amount,
            } => Certificate::Reg(stake_credential_wrapper.into_inner(), amount),
            CertificateKind::UnReg {
                stake_credential_wrapper,
                amount,
            } => Certificate::UnReg(stake_credential_wrapper.into_inner(), amount),

            CertificateKind::VoteDeleg {
                stake_credential_wrapper,
                drep_wrapper,
            } => Certificate::VoteDeleg(
                stake_credential_wrapper.into_inner(),
                drep_wrapper.into_inner(),
            ),

            CertificateKind::StakeVoteDeleg {
                stake_credential_wrapper,
                pool_key_hash,
                drep_wrapper,
            } => Certificate::StakeVoteDeleg(
                stake_credential_wrapper.into_inner(),
                parse_pool_key_hash(&pool_key_hash)?,
                drep_wrapper.into_inner(),
            ),

            CertificateKind::StakeRegDeleg {
                stake_credential_wrapper,
                pool_key_hash,
                amount,
            } => Certificate::StakeRegDeleg(
                stake_credential_wrapper.into_inner(),
                parse_pool_key_hash(&pool_key_hash)?,
                amount,
            ),

            CertificateKind::VoteRegDeleg {
                stake_credential_wrapper,
                drep_wrapper,
                amount,
            } => Certificate::VoteRegDeleg(
                stake_credential_wrapper.into_inner(),
                drep_wrapper.into_inner(),
                amount,
            ),

            CertificateKind::StakeVoteRegDeleg {
                stake_credential_wrapper,
                pool_key_hash,
                drep_wrapper,
                amount,
            } => Certificate::StakeVoteRegDeleg(
                stake_credential_wrapper.into_inner(),
                parse_pool_key_hash(&pool_key_hash)?,
                drep_wrapper.into_inner(),
                amount,
            ),

            CertificateKind::AuthCommitteeHot {
                committee_cold_cred,
                committee_hot_cred,
            } => Certificate::AuthCommitteeHot(
                committee_cold_cred.into_inner(),
                committee_hot_cred.into_inner(),
            ),

            CertificateKind::ResignCommitteeCold {
                committee_cold_cred,
                anchor_wrapper,
            } => Certificate::ResignCommitteeCold(
                committee_cold_cred.into_inner(),
                Nullable::from(anchor_wrapper.map(|a| a.into_inner())),
            ),

            CertificateKind::RegDRepCert {
                drep_cred,
                amount,
                anchor_wrapper,
            } => Certificate::RegDRepCert(
                drep_cred.into_inner(),
                amount,
                Nullable::from(anchor_wrapper.map(|a| a.into_inner())),
            ),

            CertificateKind::UnRegDRepCert { drep_cred, amount } => {
                Certificate::UnRegDRepCert(drep_cred.into_inner(), amount)
            }

            CertificateKind::UpdateDRepCert {
                drep_cred,
                anchor_wrapper,
            } => Certificate::UpdateDRepCert(
                drep_cred.into_inner(),
                Nullable::from(anchor_wrapper.map(|a| a.into_inner())),
            ),
        };
        Ok(Self {
            inner: pallas_certificate,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let certificate = Certificate::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self { inner: certificate })
    }
}

impl IntoInner<Certificate> for CertificateWrapper {
    fn into_inner(&self) -> Certificate {
        self.inner.clone()
    }
}

#[cfg(test)]
mod tests {

    use crate::wrapper::stake_credential::StakeCredentialKind;

    use super::*;

    #[test]
    fn test_certificate_wrapper_encode_decode() {
        // Example: StakeRegistration certificate
        let stake_credential = StakeCredentialWrapper::new(StakeCredentialKind::AddrKeyhash(
            "276fd18711931e2c0e21430192dbeac0e458093cd9d1fcd7210f64b3".to_string(),
        ))
        .expect("valid stake credential");

        let cert_kind = CertificateKind::StakeRegistration {
            stake_credential_wrapper: stake_credential,
        };

        let wrapper =
            CertificateWrapper::new(cert_kind).expect("should create certificate wrapper");

        let encoded = wrapper.encode();
        let decoded = CertificateWrapper::decode(encoded).expect("should decode");

        assert_eq!(wrapper, decoded);
    }

    #[test]
    #[should_panic]
    fn test_certificate_wrapper_invalid_keyhash() {
        // Example: StakeRegistration with invalid keyhash
        let stake_credential =
            StakeCredentialWrapper::new(StakeCredentialKind::AddrKeyhash("invalid".to_string()));

        assert!(
            stake_credential.is_err(),
            "should fail with invalid keyhash"
        );

        let cert_kind = CertificateKind::StakeRegistration {
            stake_credential_wrapper: stake_credential.unwrap(),
        };

        let wrapper = CertificateWrapper::new(cert_kind);
        assert!(
            wrapper.is_err(),
            "should fail to create certificate wrapper with invalid keyhash"
        );
    }
}
