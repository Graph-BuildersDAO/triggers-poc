mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const GRT_TRACKED_CONTRACT: [u8; 20] = hex!("c944e90c64b2c07662a292be6244bdf05cda44a7");

fn map_grt_events(blk: &eth::Block, events: &mut contract::Events) {
    events.grt_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::GrtApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_minter_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::MinterAdded::match_and_decode(log) {
                        return Some(contract::GrtMinterAdded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_minter_removeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::MinterRemoved::match_and_decode(log) {
                        return Some(contract::GrtMinterRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_new_ownerships.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::NewOwnership::match_and_decode(log) {
                        return Some(contract::GrtNewOwnership {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_new_pending_ownerships.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::NewPendingOwnership::match_and_decode(log) {
                        return Some(contract::GrtNewPendingOwnership {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::GrtTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}

fn db_grt_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.grt_approvals.iter().for_each(|evt| {
        tables
            .create_row("grt_approval", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.grt_minter_addeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_added", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_minter_removeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_removed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_new_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_ownership", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_new_pending_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_pending_ownership", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_transfers.iter().for_each(|evt| {
        tables
            .create_row("grt_transfer", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}


fn graph_grt_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.grt_approvals.iter().for_each(|evt| {
        tables
            .create_row("grt_approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.grt_minter_addeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_added", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_minter_removeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_removed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_new_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_ownership", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_new_pending_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_pending_ownership", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_transfers.iter().for_each(|evt| {
        tables
            .create_row("grt_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
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
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_grt_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_grt_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
