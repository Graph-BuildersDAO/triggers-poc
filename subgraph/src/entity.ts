import { BigDecimal, BigInt } from "@graphprotocol/graph-ts";
import { Account, Transfer } from "../generated/schema";

export function getOrCreateAccount(address: string): Account {
  let account = Account.load(address);
  if (!account) {
    account = new Account(address);
    account.grt_balance = BigInt.zero();
  }
  return account;
}

export function createAndSaveTransfer(
  transferId: string,
  txHash: string,
  evtIndex: BigInt,
  blockTime: string,
  blockNumber: BigInt,
  from: string,
  to: string,
  value: BigDecimal
): void {
  let transfer = new Transfer(transferId);
  transfer.evt_tx_hash = txHash;
  transfer.evt_index = evtIndex;
  transfer.evt_block_time = blockTime;
  transfer.evt_block_number = blockNumber;
  transfer.from = from;
  transfer.to = to;
  transfer.value = value;
  transfer.save();
}
