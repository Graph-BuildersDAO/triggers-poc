import { BigDecimal, BigInt, log } from "@graphprotocol/graph-ts";
import * as assembly from "./assembly";
import { Transfer as TransferEvent } from "../generated/grt/GRT";
import { ADDRESS_ZERO, ADDRESS_ZERO_STRING } from "./constants";
import { createAndSaveTransfer, getOrCreateAccount } from "./entity";

export function handleTransfers(bytes: Uint8Array): void {
  let transfers = assembly.contract.v1.Transfers.decode(bytes.buffer);

  if (transfers.transfers.length == 0) {
    log.info("No transfers found", []);
    return;
  }

  for (let i = 0; i < transfers.transfers.length; i++) {
    let transferData = transfers.transfers[i];
    let transferId =
      transferData.evt_tx_hash.toString() +
      "-" +
      transferData.evt_index.toString();

    createAndSaveTransfer(
      transferId,
      transferData.evt_tx_hash.toString(),
      BigInt.fromU32(transferData.evt_index),
      transferData.evt_block_time.seconds.toString(),
      BigInt.fromU64(transferData.evt_block_number),
      transferData.from,
      transferData.to,
      BigDecimal.fromString(transferData.value)
    );

    if (transferData.from != ADDRESS_ZERO_STRING) {
      let fromAccount = getOrCreateAccount(transferData.from);
      fromAccount.grt_balance = BigInt.fromString(transferData.from_balance);
      fromAccount.save();
    }

    if (transferData.to != ADDRESS_ZERO_STRING) {
      let toAccount = getOrCreateAccount(transferData.to);
      toAccount.grt_balance = BigInt.fromString(transferData.to_balance);
      toAccount.save();
    }
  }
}

export function handleTransfer(event: TransferEvent): void {
  const to = event.params.to;
  const from = event.params.from;
  const value = event.params.value;

  let receipt = event.receipt;
  let txHash = "";
  if (receipt != null) {
    txHash = receipt.transactionHash.toHexString();
  }
  let transferId = txHash + "-" + event.logIndex.toString();

  createAndSaveTransfer(
    transferId,
    txHash,
    event.logIndex,
    event.block.timestamp.toString(),
    event.block.number,
    from.toHexString(),
    to.toHexString(),
    value.toBigDecimal()
  );

  if (from != ADDRESS_ZERO) {
    let fromAccount = getOrCreateAccount(from.toHexString());
    fromAccount.grt_balance = fromAccount.grt_balance.minus(value);
    fromAccount.save();
  }

  if (to != ADDRESS_ZERO) {
    let toAccount = getOrCreateAccount(to.toHexString());
    toAccount.grt_balance = toAccount.grt_balance.plus(value);
    toAccount.save();
  }
}
