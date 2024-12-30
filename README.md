# Escrow Fight Contract

This smart contract is implemented in Rust for the MultiversX blockchain and functions as an escrow for a fighting game between Soldiers. Users can create a game, join as competitors, and potentially win an entrance fee prize based on their Soldier's attributes. The contract uses randomness and Soldier attributes to determine the winner.

## Features

- **Game Initiation**: Users can create a game by sending their Soldier and setting an entrance fee.
- **Competitor Joining**: Another user can join the game by paying the entrance fee and sending their Soldier.
- **Random Winner Determination**: The winner is determined randomly, with a higher chance of winning for Soldiers with better attributes.
- **Prize Distribution**: The winner receives the total entrance fees.

## Prerequisites

- [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed.
- MultiversX CLI tools for deploying and interacting with smart contracts.

## Project Structure
escrow_fight_contract/
├── Cargo.toml
├── src
│ └── lib.rs
└── README.md

Copy

## Setup and Deployment

1. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd escrow_fight_contract
Build the Contract: Compile the smart contract to WebAssembly.

cargo build --release --target=wasm32-unknown-unknown
Deploy the Contract: Use MultiversX tools to deploy the compiled Wasm file to the MultiversX Devnet or Mainnet.

Interact with the Contract: Use the MultiversX Devnet Explorer or CLI to call endpoints such as createGame, joinGame, and startFight.

Contract Usage
Create Game
Endpoint: createGame
Parameters:
initiator_soldier: Details of the Soldier being sent to the contract.
entrance_fee: The fee required to enter the game.
Join Game
Endpoint: joinGame
Parameters:
initiator: Address of the game initiator.
competitor_soldier: Details of the competitor's Soldier.
Start Fight
Endpoint: startFight
Parameters:
initiator: Address of the game initiator.
Verification
Code Verification: Review the contract code in src/lib.rs.
ABI Verification: Check the contract's ABI for endpoint definitions and parameter types.