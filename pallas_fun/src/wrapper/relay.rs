use crate::utils::{IntoInner, option_hex_string_to_option_bytes};
use hex;
use pallas::codec::minicbor::{self, Decode, Encode};
use pallas::codec::utils::Nullable;
use pallas::ledger::primitives::{Fragment, Relay};
use serde::{Deserialize, Serialize};

pub enum RelayKind {
    SingleHostAddr(Option<u32>, Option<String>, Option<String>),
    SingleHostName(Option<u32>, String),
    MultiHostName(String),
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct RelayWrapper {
    #[n(0)]
    inner: Relay,
}

impl RelayWrapper {
    pub fn new(relay: RelayKind) -> Result<Self, String> {
        let pallas_relay = match relay {
            RelayKind::SingleHostAddr(port, ipv4, ipv6) => Relay::SingleHostAddr(
                Nullable::from(port),
                Nullable::from(option_hex_string_to_option_bytes(ipv4)),
                Nullable::from(option_hex_string_to_option_bytes(ipv6)),
            ),
            RelayKind::SingleHostName(port, dns_name) => {
                Relay::SingleHostName(Nullable::from(port), dns_name)
            }
            RelayKind::MultiHostName(dns_name) => Relay::MultiHostName(dns_name),
        };

        Ok(Self {
            inner: pallas_relay,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(self.into_inner().encode_fragment().unwrap())
    }

    pub fn decode(hex_string: String) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decode error: {}", e))?;
        let pallas_relay =
            Relay::decode_fragment(&bytes).map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self {
            inner: pallas_relay,
        })
    }

    // pub fn into_inner(&self) -> Relay {
    //     self.pallas_relay.clone()
    // }
}

impl IntoInner<Relay> for RelayWrapper {
    fn into_inner(&self) -> Relay {
        self.inner.clone()
    }
}
