use substreams::Hex;
use substreams_ethereum::block_view::CallView;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::collections::HashMap;
use substreams::scalar::BigInt;

use crate::abi::grt_contract::events::Transfer;

// CONSTANTS for `map_hashes_to_addresses` function
const EXPECTED_PREIMAGE_LENGTH: usize = 128;
const ADDRESS_START: usize = 24;
const ADDRESS_END: usize = 64;
const PADDING_START: usize = 64;
const PADDING_END: usize = 126;
const ZERO_PADDING: &str = "00000000000000000000000000000000000000000000000000000000000000";

pub fn map_hashes_to_addresses(call: &CallView) -> HashMap<Vec<u8>, Vec<u8>> {
    let mut hash_to_address = HashMap::new();

    for (hash, preimage) in &call.call.keccak_preimages {
        // The keccak preimage consists of a 32 byte address concantenated with a 32 byte storage slot index.
        // An ethereum is 20 bytes long so it is padded to 32 bytes with leading zeroes in the preimage.
        // The storage slot also is padded to 32 bytes with leading zeroes in the preimage.
        // Check if the preimage is 64 bytes long and that the second 32 bytes consists of leading zeroes for the padding before the storage slot.
        if preimage.len() != EXPECTED_PREIMAGE_LENGTH
            || &preimage[PADDING_START..PADDING_END] != ZERO_PADDING
        {
            continue;
        }

        let address_slice = &preimage[ADDRESS_START..ADDRESS_END];

        match (Hex::decode(hash), Hex::decode(address_slice)) {
            (Ok(decoded_hash), Ok(decoded_address)) => {
                hash_to_address.insert(decoded_hash, decoded_address);
            }
            (Err(e), _) | (_, Err(e)) => {
                substreams::log::info!("Failed to decode hash or address: {}", e);
                continue;
            }
        }
    }

    hash_to_address
}

pub fn extract_balances_from_call(
    call: &CallView,
    transfer: &Transfer,
    hash_to_address: &HashMap<Vec<u8>, Vec<u8>>,
) -> (BigInt, BigInt) {
    let mut from_balance = BigInt::zero();
    let mut to_balance = BigInt::zero();

    for change in &call.call.storage_changes {
        let old_value = BigInt::from_signed_bytes_be(&change.old_value);
        let new_value = BigInt::from_signed_bytes_be(&change.new_value);
        let diff = new_value.clone() - old_value;

        if let Some(address) = hash_to_address.get(&change.key) {
            if diff.absolute() == transfer.value {
                // Determine if this is the 'from' or 'to' address based on the sign of diff
                if diff < BigInt::zero() && address == &transfer.from {
                    from_balance = new_value;
                } else if diff > BigInt::zero() && address == &transfer.to {
                    to_balance = new_value;
                }
            }
        }
    }

    (from_balance, to_balance)
}
