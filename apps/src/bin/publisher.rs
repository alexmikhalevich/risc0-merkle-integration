// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use alloy_sol_types::{sol, SolInterface};
use anyhow::Result;
use ethers::prelude::*;
use methods::MERKLE_ELF;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use apps::input::{load_config, read_input_data};

// `IMerkleRoot` interface automatically generated via the alloy `sol!` macro.
sol! {
    interface IMerkleRoot {
        function set(bytes root, bytes calldata seal);
    }
}

/// Wrapper of a `SignerMiddleware` client to send transactions to the given
/// contract's `Address`.
pub struct TxSender {
    chain_id: u64,
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    contract: Address,
}

impl TxSender {
    /// Creates a new `TxSender`.
    pub fn new(chain_id: u64, rpc_url: &str, private_key: &str, contract: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        let contract = contract.parse::<Address>()?;

        Ok(TxSender {
            chain_id,
            client,
            contract,
        })
    }

    /// Send a transaction with the given calldata.
    pub async fn send(&self, calldata: Vec<u8>) -> Result<Option<TransactionReceipt>> {
        let tx = TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(self.contract)
            .from(self.client.address())
            .data(calldata);

        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;

        log::info!("Transaction receipt: {:?}", &tx);

        Ok(tx)
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let config = load_config()?;

    // Create a new transaction sender using the parsed arguments.
    let tx_sender = TxSender::new(
        config.chain_id,
        &config.rpc_url,
        &config.eth_wallet_private_key,
        &config.contract,
    )?;

    let input = read_input_data(&config)?;
    let env = ExecutorEnv::builder().write(&input)?.build()?;

    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            MERKLE_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;

    // Encode the seal with the selector.
    let seal = groth16::encode(receipt.inner.groth16()?.seal.clone())?;

    // Extract the journal from the receipt.
    let journal = receipt.journal.bytes.clone();

    // Construct function call: Using the IMerkleRoot interface, the application constructs
    // the ABI-encoded function call for the set function of the MerkleRoot contract.
    // This call includes the verified root, the post-state digest, and the seal (proof).
    let calldata = IMerkleRoot::IMerkleRootCalls::set(IMerkleRoot::setCall {
        root: journal,
        seal: seal.into(),
    })
    .abi_encode();

    // Initialize the async runtime environment to handle the transaction sending.
    let runtime = tokio::runtime::Runtime::new()?;

    // Send transaction: Finally, the TxSender component sends the transaction to the Ethereum blockchain,
    // effectively calling the set function of the MerkleRoot contract with the verified root and proof.
    runtime.block_on(tx_sender.send(calldata))?;

    Ok(())
}
