import {
  Address,
  BigDecimal,
  BigInt,
  Bytes,
  log,
} from "@graphprotocol/graph-ts";
import * as assembly from "./assembly";
import { Transfer } from "../generated/schema";

export function handleTransfers(bytes: Uint8Array): void {
  let transfers = assembly.contract.v1.Transfers.decode(bytes.buffer);
  if (transfers.transfers.length == 0) {
    log.info("No transfers found", []);
    return;
  } else {
    for (let i = 0; i < transfers.transfers.length; i++) {
      let transferData = transfers.transfers[i];
      let transferId =
        transferData.evt_tx_hash.toString() +
        "-" +
        transferData.evt_index.toString();

      let entity = new Transfer(transferId);
      entity.evt_tx_hash = transferData.evt_tx_hash.toString();
      entity.evt_index = BigInt.fromU32(transferData.evt_index);
      entity.evt_block_time = transferData.evt_block_time.seconds.toString();
      entity.evt_block_number = BigInt.fromU64(transferData.evt_block_number);
      entity.from = transferData.from;
      entity.to = transferData.to;
      entity.value = BigDecimal.fromString(transferData.value);
      entity.save();
    }
  }
}
