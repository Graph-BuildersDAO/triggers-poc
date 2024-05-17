mod abi;
mod pb;
use abi::grt_contract::events::Transfer;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::Event;
use substreams_ethereum::{block_view::CallView, pb::eth::v2 as eth};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::{collections::HashMap, str::FromStr};
use substreams::scalar::{BigDecimal, BigInt};

substreams_ethereum::init!();

const GRT_TRACKED_CONTRACT: [u8; 20] = hex!("c944e90c64b2c07662a292be6244bdf05cda44a7");

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<contract::Transfers, substreams::errors::Error> {
    let mut transfers = contract::Transfers::default();

    transfers.transfers.append(
        &mut blk
            .transactions()
            .flat_map(|trx| {
                trx.logs_with_calls()
                    .filter(|(log, _)| log.address == GRT_TRACKED_CONTRACT)
                    .filter_map(|(log, call)| {
                        if let Some(transfer) =
                            abi::grt_contract::events::Transfer::match_and_decode(log)
                        {
                            let hash_to_address = map_hashes_to_addresses(&call);

                            let (from_balance, to_balance) =
                                extract_balances_from_call(&call, &transfer, &hash_to_address);

                            return Some(contract::Transfer {
                                evt_tx_hash: format!("0x{}", Hex::encode(&call.transaction.hash)),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: format!("0x{}", Hex::encode(transfer.from)),
                                to: format!("0x{}", Hex::encode(transfer.to)),
                                value: transfer.value.to_string(),
                                from_balance: from_balance.to_string(),
                                to_balance: to_balance.to_string(),
                            });
                        }
                        None
                    })
            })
            .collect(),
    );

    Ok(transfers)
}

fn graph_grt_out(transfers: &contract::Transfers, tables: &mut EntityChangesTables) {
    transfers.transfers.iter().for_each(|evt| {
        tables
            .create_row(
                "grt_transfer",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", &evt.from)
            .set("to", &evt.to)
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}

#[substreams::handlers::map]
fn graph_out(events: contract::Transfers) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_grt_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}

// CONSTANTS for `map_hashes_to_addresses` function
const EXPECTED_PREIMAGE_LENGTH: usize = 128;
const ADDRESS_START: usize = 24;
const ADDRESS_END: usize = 64;
const PADDING_START: usize = 64;
const PADDING_END: usize = 126;
const ZERO_PADDING: &str = "00000000000000000000000000000000000000000000000000000000000000";

fn map_hashes_to_addresses(call: &CallView) -> HashMap<Vec<u8>, Vec<u8>> {
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

fn extract_balances_from_call(
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
