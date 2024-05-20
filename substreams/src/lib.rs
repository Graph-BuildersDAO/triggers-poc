mod abi;
mod pb;
mod utils;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;
use utils::{extract_balances_from_call, map_hashes_to_addresses};

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
