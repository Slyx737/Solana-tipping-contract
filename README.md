# Solana Tipping Contract

This repository contains a Solana smart contract for a tipping system. The contract allows anyone to deposit SOL into it, and only the owner can send tips. The contract is written in Rust and uses the Solana Program Library (SPL) for interacting with the Solana blockchain.

## Features

- **Deposit Funds:** Any user can deposit funds into the contract.
- **Send Tips:** Only the owner of the contract can send tips to other users.
- **Initialize Contract:** The contract must be initialized before it can be used. The owner of the contract is set during initialization.

## Getting Started

To build the contract, you need to have Rust and the Solana tool suite installed on your machine. Once you have these prerequisites, you can clone the repository and build the contract:

Use `git clone https://github.com/yourusername/solana-tipping-contract.git` to clone the repository.

Navigate to the project directory with `cd solana-tipping-contract`.

Build the contract with `cargo build-bpf`.

## Testing

This repository includes a test suite for the tipping contract. To run the tests, use the following command:

Run the tests with `cargo test`.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## License

This project is licensed under the terms of the MIT license.
