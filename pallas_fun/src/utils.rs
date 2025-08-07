use std::str::FromStr;

use pallas::{
    codec::utils::{Bytes, Set},
    ledger::primitives::{
        AddrKeyhash, DatumHash, PoolKeyhash, PoolMetadataHash, RationalNumber, ScriptHash,
        UnitInterval, VrfKeyhash,
    },
};

// key hash parser from &str to key hashes
pub fn parse_address_key_hash(address_key_hash_str: &str) -> Result<AddrKeyhash, String> {
    address_key_hash_str
        .parse()
        .map_err(|_| "Invalid address key hash length".to_string())
}

pub fn parse_script_hash(script_hash_str: &str) -> Result<ScriptHash, String> {
    script_hash_str
        .parse()
        .map_err(|_| "Invalid script hash length".to_string())
}

pub fn parse_pool_key_hash(pool_keyhash_str: &str) -> Result<PoolKeyhash, String> {
    pool_keyhash_str
        .parse()
        .map_err(|_| "Invalid pool key hash length".to_string())
}

pub fn parse_vrf_key_hash(vrf_keyhash_str: &str) -> Result<VrfKeyhash, String> {
    vrf_keyhash_str
        .parse()
        .map_err(|_| "Invalid VRF key hash length".to_string())
}

pub fn parse_pool_metadata_hash(pool_metadata_key_hash: &str) -> Result<PoolMetadataHash, String> {
    pool_metadata_key_hash
        .parse()
        .map_err(|_| "Invalid pool metadata key hash length".to_string())
}

pub fn parse_datum_hash(datum_hash: &str) -> Result<DatumHash, String> {
    datum_hash
        .parse()
        .map_err(|_| "Invalid datum hash length".to_string())
}

// UnitInterval parser from u64 to UnitInterval
pub fn parse_rational_number(numerator: u64, denominator: u64) -> Result<UnitInterval, String> {
    if denominator == 0 {
        return Err("Denominator cannot be zero".to_string());
    }
    Ok(RationalNumber {
        numerator,
        denominator,
    })
}

// pub fn parse_unit_interval(numerator: u64, denominator: u64) -> Result<UnitInterval, String> {
//     if denominator == 0 {
//         return Err("Denominator cannot be zero".to_string());
//     }
//     Ok(UnitInterval {
//         numerator,
//         denominator,
//     })
// }

pub fn parse_vec_string_to_set_addr_keyhash(
    inputs: Vec<String>,
) -> Result<Set<AddrKeyhash>, String> {
    let mut result = Vec::with_capacity(inputs.len());
    for s in inputs {
        let keyhash = parse_address_key_hash(&s)?;
        result.push(keyhash);
    }
    Ok(Set::from(result))
}

// pub fn parse_bytes(byte_str: &str) -> Result<Bytes, String> {
//     Bytes::from_str(byte_str).map_err(|_| "Invalid byte length".to_string())
// }

pub fn option_hex_string_to_option_bytes(opt: Option<String>) -> Option<Bytes> {
    opt.and_then(|s| Bytes::from_str(&s).ok())
}

pub fn parse_vec_wrapper_to_set<T, U>(inputs: Vec<T>) -> Set<U>
where
    T: IntoInner<U>,
{
    Set::from(
        inputs
            .into_iter()
            .map(|w| w.into_inner())
            .collect::<Vec<U>>(),
    )
}
pub trait IntoInner<T> {
    fn into_inner(&self) -> T;
}
