CREATE TABLE IF NOT EXISTS grt_approval (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS grt_minter_added (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "account" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS grt_minter_removed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "account" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS grt_new_ownership (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "from" VARCHAR(40),
    "to" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS grt_new_pending_ownership (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "from" VARCHAR(40),
    "to" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS grt_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");

