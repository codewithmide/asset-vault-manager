# Asset Manager's Vault

Asset Manager's Vault is a Solana program built with Anchor that allows customers to deposit SPL tokens of their choice into a secure vault. The key feature of this vault is that the vault manager cannot withdraw the deposited funds, ensuring the security of the users' assets.

## Features

- Secure token deposits
- User-controlled withdrawals
- Support for multiple SPL tokens
- Vault manager cannot access deposited funds

## Prerequisites

Before you begin, ensure you have installed the following:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/en/download/) and [Yarn](https://yarnpkg.com/getting-started/install)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/codewithmide/asset-vault-manager.git
   cd asset-vault-manager
   ```

2. Install dependencies:

   ```bash
   yarn install
   ```

3. Build the program:

   ```bash
   anchor build
   ```

4. Deploy the program (make sure you have a Solana keypair set up):

   ```bash
   anchor deploy
   ```

5. Update the program ID in `lib.rs` and `Anchor.toml` with the newly generated program ID.

## Usage

### Running Tests

To run the test suite:

```bash
anchor test
```

### Interacting with the Program

You can interact with the program using the Anchor TypeScript client. Here's a basic example of how to initialize a vault and deposit tokens:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AssetManagerVault } from "../target/types/asset_manager_vault";

// Set up the provider and program
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.AssetManagerVault as Program<AssetManagerVault>;

// Initialize the vault
await program.methods
  .initializeVault()
  .accounts({
    vault: vaultAccount,
    authority: vaultManager.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([vaultManager])
  .rpc();

// Deposit tokens
await program.methods
  .deposit(new anchor.BN(amount))
  .accounts({
    vault: vaultAccount,
    vaultTokenAccount: vaultTokenAccount,
    depositorTokenAccount: depositorTokenAccount,
    depositor: depositor.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([depositor])
  .rpc();
```

## Project Structure

- `programs/asset_manager_vault/src/lib.rs`: Main program logic
- `tests/asset_manager_vault.ts`: TypeScript tests
- `Anchor.toml`: Anchor configuration file
- `Cargo.toml`: Rust dependencies

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This program is provided as-is and should be audited and tested thoroughly before use in any production environment. The authors are not responsible for any loss of funds or other damages that may occur from using this program.
