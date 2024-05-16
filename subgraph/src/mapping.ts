import {
  Address,
  BigDecimal,
  BigInt,
  Bytes,
  log,
} from "@graphprotocol/graph-ts";
import * as assembly from "./assembly";
import { Account, Transfer } from "../generated/schema";

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

      let transfer = new Transfer(transferId);
      transfer.evt_tx_hash = transferData.evt_tx_hash.toString();
      transfer.evt_index = BigInt.fromU32(transferData.evt_index);
      transfer.evt_block_time = transferData.evt_block_time.seconds.toString();
      transfer.evt_block_number = BigInt.fromU64(transferData.evt_block_number);
      transfer.from = transferData.from;
      transfer.to = transferData.to;
      transfer.value = BigDecimal.fromString(transferData.value);
      transfer.save();

      let fromAccount = Account.load(transferData.from);
      if (fromAccount == null) {
        fromAccount = new Account(transferData.from);
      }
      fromAccount.grt_balance = BigInt.fromString(transferData.from_balance);
      fromAccount.save();

      let toAccount = Account.load(transferData.to);
      if (toAccount == null) {
        toAccount = new Account(transferData.to);
      }
      toAccount.grt_balance = BigInt.fromString(transferData.to_balance);
      toAccount.save();
    }
  }
}
