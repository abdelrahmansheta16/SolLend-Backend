# SolLend Solana Program

This repository contains the Solana program (smart contract) for SolLend, a decentralized NFT-backed lending platform on the Solana blockchain.

## Overview

SolLend allows users to use their NFTs as collateral to borrow cryptocurrency. This Solana program handles the core functionality of the lending process, including creating loan offers, accepting loans, repaying loans, and liquidating collateral if necessary.

## Features

- Create loan offers specifying amount, interest rate, and duration
- Accept loans by providing NFT collateral
- Repay loans to reclaim NFT collateral
- Automatic liquidation of collateral for defaulted loans
- Manage collection pools for different NFT collections

## Prerequisites

- Rust 1.70.0 or later
- Solana CLI 1.16.0 or later
- Anchor Framework 0.28.0 or later

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/sollend-solana-program.git
   cd sollend-solana-program
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Build the program:
   ```
   anchor build
   ```

4. Test the program:
   ```
   anchor test
   ```

## Program Structure

- `lib.rs`: Main entry point for the Solana program
- `instructions/`: Contains modules for each instruction (create_offer, accept_loan, repay_loan, etc.)
- `state/`: Defines the account structures used in the program
- `errors.rs`: Custom error types for the program

## Key Accounts

- `CollectionPool`: Stores information about each NFT collection pool
- `Offer`: Represents a loan offer
- `ActiveLoan`: Represents an active loan
- `Vault`: Holds the borrowed funds

## Deployment

To deploy the program to Solana devnet:

1. Set up your Solana CLI config for devnet:
   ```
   solana config set --url https://api.devnet.solana.com
   ```

2. Deploy the program:
   ```
   anchor deploy
   ```

3. Update the `declare_id!` macro in `lib.rs` with the new program ID

## Testing

Run the test suite with:

```
anchor test
```

This will run through various scenarios including creating offers, accepting loans, repaying loans, and handling liquidations.

## Security Considerations

- This program handles financial transactions and NFT ownership. It has been designed with security in mind, but we recommend a thorough audit before mainnet deployment.
- Ensure proper access controls and validation checks are in place for all critical operations.

## Contributing

We welcome contributions to the SolLend Solana program! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For any queries regarding the SolLend Solana program, please open an issue in this repository or contact the maintainers at [abdelrahman.sheta16@gmail.com].

