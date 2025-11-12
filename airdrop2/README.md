# outline of the codebase in brief

### TL;DR
Unit tests written in Rust demonstrating keypair generation, airdrops, transfers, key export between Base58 and JSON, also completes the Turbin3 prereq2 submission

### Working of each test

- Keygen() : it generates a new keypair and prints in JSON byte array and base58 format

- claim_airdrop() : sol airdrop on devnet from dev-wallet.json(keypair.to_bytes()) using solana_client::rpc_client::RpcClient

- transfer_sol() : Signs and verifies a message with the keypair, fetches latest blockhash, computes  get_fee_for_message after fees is calculated it sends the remaining bal to turbin3 wallet to not waste dev sols

- base58_to_wallet() & wallet_to_base58() : do exactly like their names, the key is handling the inputs gracefully. bs58 does the heavy lifting of decoding and encoding the conversions

- submit_rs() : derives two PDAs prereqs PDA and authority PDA, ensuring the derived addresses match the 
ANchor constraints
    ```
    AccountMeta::new(signer.pubkey(), true),
    AccountMeta::new(collection, false),
    AccountMeta::new_readonly(authority, false),
    ```
    new is for accounts that the instruction writes to and new_readonly for accounts that are read-only. The true
    flag indicates the account must sign the transaction

    It constructs, signs, and dispatches a transaction with recent blockhash
    ```
    &[instruction],
    Some(&signer.pubkey()),
    &[&signer, &mint],
    blockhash,
    ```
    the prereqs PDA is then updated succesfully using "send_and_confirm_transactions"
