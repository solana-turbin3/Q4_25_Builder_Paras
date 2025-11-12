import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, tokenMetadataInitialize, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("YYCW7pXnJrKnvFv1CoSojZVPN9PXq3zk57byYjq86yU");

// Recipient address
const to = new PublicKey("78cesAi6taEvHbcHpj2LsGEoQZcCKYpvqN29SRa3APTR");
const token_decimals = 1_000_000n;

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ata = await getOrCreateAssociatedTokenAccount (
            connection,
            keypair,
            mint,
            keypair.publicKey,
            false,

        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const ata2 = await getOrCreateAssociatedTokenAccount (
            connection,
            keypair,
            mint,
            to,
            false,
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const transferToken = await transfer (
            connection,
            keypair,
            ata.address,
            ata2.address,
            keypair.publicKey,
            token_decimals * 1n,     
            
        )
        console.log(`Your transfer txid: ${transferToken}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();