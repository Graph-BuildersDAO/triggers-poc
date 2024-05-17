
# Substreams Triggers PoC

As of `graph-node` [v0.34.0](https://github.com/graphprotocol/graph-node/releases/tag/v0.34.0), the output of Substreams map modules can now trigger Subgraph mappings.

This repository is an initial look at a specific use case for these triggers, tracking the balances of the GRT token for accounts involved in `Transfer` events emitted from the ERC20 contract.

## Substreams

The `substreams` folder contains the substreams package and its relevant code. Inside `lib.rs` is the substreams map module utilised by the Subgraph as a trigger, this module is called `map_transfers`. This module takes in an `Block` and outputs a `Transfers` protobuf message which is then decoded by the Subgraph handler.

Before deploying the Subgraphs the Substream package needs to be built and packed. This can be done via the `make pack` command whilst inside the `substreams` folder.

You can also run the Substreams module in isolation via the `make run` or `make gui` commands.

## Subgraphs

The repository contains two Subgraphs inside the subgraph folder, which allow us to benchmark and compare metrics for two methods of storing account balances. These can be deployed using their corresponding build and deploy commands (`yarn run build-triggers && yarn run deploy-triggers`).

- Normal Subgraph
    - Event handler for GRT `Tranfer` event
    - Updates account balances the typical way using `event.params.value` and incrementing/decrementing the relevant `Account` entities balance.
- Substreams Trigger Powered Subgraph
    - Substreams map module utilised by Subgraph as a datasource
    - Trigger handler decodes `Transfer` events outputted from Substreams module and creates relevant `Transfer` and `Account` entities.
    - Balances are retrieved via storage changes in the Substreams module, and therefore now incrementing/decrementing is required.


## Useful Links

- https://substreams.streamingfast.io/
- https://github.com/graphprotocol/graph-node/releases/tag/v0.34.0
- https://github.com/kmjones1979/sps-triggers
- https://github.com/graphprotocol/graph-tooling/tree/substreams-triggers-as-subgraph-mappings/examples/near-substreams-triggers-as-subgraph-mappings