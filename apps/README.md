# Apps

In typical applications, an off-chain app is needed to do two main actions:

* Produce a proof e.g. by sending a proof request to [Bonsai].
* Send a transaction to Ethereum to execute your on-chain logic.

This application executes these steps.

## Publisher

The [`publisher` CLI][publisher] sends an off-chain proof request to the [Bonsai] proving service, and publishes the received proofs to the deployed app contract.

### Usage

Run the `publisher` with:

```sh
cargo run --bin publisher --config config.yaml
```

[publisher]: ./src/bin/publisher.rs
[Bonsai]: https://dev.bonsai.xyz/
