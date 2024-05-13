mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::{collections::HashMap, str::FromStr};
use substreams::scalar::{BigDecimal, BigInt};

substreams_ethereum::init!();

const GRT_TRACKED_CONTRACT: [u8; 20] = hex!("c944e90c64b2c07662a292be6244bdf05cda44a7");

fn map_grt_events(blk: &eth::Block, events: &mut contract::Events) {
    events.grt_transfers.append(
        &mut blk
            .transactions()
            .flat_map(|trx| {
                trx.logs_with_calls()
                    .filter(|(log, _)| log.address == GRT_TRACKED_CONTRACT)
                    .filter_map(|(log, call)| {
                        if let Some(transfer) =
                            abi::grt_contract::events::Transfer::match_and_decode(log)
                        {
                            let mut hash_to_address = HashMap::new();

                            for (hash, preimage) in &call.call.keccak_preimages {
                                if preimage.len() != 128 {
                                    continue;
                                }

                                if &preimage[64..126] != "00000000000000000000000000000000000000000000000000000000000000" {
                                    continue;
                                }

                                let addr = &preimage[24..64];

                                if let Ok(hash) = Hex::decode(hash) {
                                    if let Ok(addr) = Hex::decode(addr) {
                                        hash_to_address.insert(hash, addr);
                                    }
                                }
                            }

                            let mut from_balance = BigInt::zero();
                            let mut to_balance = BigInt::zero();

                            
                            for change in &call.call.storage_changes {
                                let old_value = BigInt::from_signed_bytes_be(&change.old_value);
                                let new_value = BigInt::from_signed_bytes_be(&change.new_value);
                                let diff = new_value.clone() - old_value;

                                let keccak_address = match hash_to_address.get(&change.key) {
                                    Some(addr) => addr,
                                    None => continue,
                                };

                                if diff.lt(&BigInt::zero()) && diff.neg() == transfer.value && keccak_address == &transfer.from {
                                    substreams::log::debug!("inside condition 1");
                                    from_balance = new_value;
                                } else if diff == transfer.value && keccak_address == &transfer.to {
                                    substreams::log::debug!("inside condition 2");
                                    to_balance = new_value;
                                }
                            }
                            

                            return Some(contract::GrtTransfer {
                                evt_tx_hash: Hex(&call.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: transfer.from,
                                to: transfer.to,
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
}

fn graph_grt_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    events.grt_transfers.iter().for_each(|evt| {
        tables
            .create_row(
                "grt_transfer",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_grt_events(&blk, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_grt_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
