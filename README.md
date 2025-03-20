# Sudoku Verifier with Zero-Knowledge Proofs

This project is a Sudoku verification system that uses [SP1](https://github.com/succinctlabs/sp1), a Zero-Knowledge Virtual Machine (ZKVM), to create zero-knowledge proofs that validate Sudoku solutions without revealing the solution itself.

## Overview

The Sudoku Verifier demonstrates how to use zero-knowledge proofs to verify the correctness of a Sudoku puzzle solution without revealing the solution. This has practical applications in scenarios where:

- You want to prove you've solved a puzzle without giving away the solution
- You need to validate solutions for a competition without revealing answers
- You want to learn and demonstrate the capabilities of ZK technology

The project consists of three main components:

1. **Core Library (`lib`)**: Contains the Sudoku validation logic
2. **ZK Program (`program`)**: The program that runs inside the SP1 ZKVM to generate proofs
3. **API & Scripts (`script`)**: An API server that handles validation requests and proof generation

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html) (Succinct's zero-knowledge virtual machine)
- Cargo with nightly toolchain for SP1 features
- At least 16GB RAM for basic proof generation (128GB for EVM-compatible proofs)

## Project Structure

```
sudoku-verifier/
├── lib/               # Core Sudoku validation logic
├── program/           # SP1 ZKVM program for proof generation
├── script/            # API server and utility scripts
├── proofs/            # Generated ZK proofs (gitignored)
├── Cargo.toml         # Workspace configuration
└── README.md          # Project documentation
```

## How It Works

1. **Validation Logic**: The system first checks if a Sudoku solution is valid using traditional methods by verifying:
   - All numbers in the puzzle (non-zero) match the solution
   - Each row contains the numbers 1-9 exactly once
   - Each column contains the numbers 1-9 exactly once
   - Each 3x3 sub-grid contains the numbers 1-9 exactly once

2. **Zero-Knowledge Proof Generation**: If the solution is valid, the system uses SP1 to generate a zero-knowledge proof attesting to the validity without revealing the actual solution.

3. **API Service**: The included API server accepts Sudoku puzzles and solutions, validates them, generates ZK proofs, and returns the validation results.

## Running the Project

### 1. Start the API Server

```sh
cd sudoku-verifier
cargo run -p sudoku-script
```

This will start an API server on port 3000 that can validate Sudoku puzzles and generate ZK proofs.

### 2. Validate a Sudoku Puzzle

Send a POST request to the API server:

```sh
curl -X POST -H "Content-Type: application/json" -d '{
  "board": [
    [5,3,0,0,7,0,0,0,0],
    [6,0,0,1,9,5,0,0,0],
    [0,9,8,0,0,0,0,6,0],
    [8,0,0,0,6,0,0,0,3],
    [4,0,0,8,0,3,0,0,1],
    [7,0,0,0,2,0,0,0,6],
    [0,6,0,0,0,0,2,8,0],
    [0,0,0,4,1,9,0,0,5],
    [0,0,0,0,8,0,0,7,9]
  ],
  "solution": [
    [5,3,4,6,7,8,9,1,2],
    [6,7,2,1,9,5,3,4,8],
    [1,9,8,3,4,2,5,6,7],
    [8,5,9,7,6,1,4,2,3],
    [4,2,6,8,5,3,7,9,1],
    [7,1,3,9,2,4,8,5,6],
    [9,6,1,5,3,7,2,8,4],
    [2,8,7,4,1,9,6,3,5],
    [3,4,5,2,8,6,1,7,9]
  ]
}' http://localhost:3000/validate-sudoku
```

The server will respond with:
```json
{
  "is_valid": true,
  "proof_generated": true
}
```

### 3. Generate Proofs Directly

You can also generate proofs without using the API:

```sh
cd sudoku-verifier/script
cargo run --release --bin main
```

### 4. Generate EVM-Compatible Proofs

For Ethereum-compatible proofs:

```sh
cd sudoku-verifier/script
cargo run --release --bin evm -- --system groth16
```

or

```sh
cargo run --release --bin evm -- --system plonk
```

**Note**: Generating EVM-compatible proofs requires at least 128GB RAM.

### 5. Get the Verification Key

To retrieve your verification key for on-chain verification:

```sh
cd sudoku-verifier/script
cargo run --release --bin vkey
```

## Using the Prover Network

For generating proofs for larger Sudoku puzzles or in a production environment, we recommend using the Succinct prover network:

1. Copy the example environment file:
   ```sh
   cp .env.example .env
   ```

2. Set the `SP1_PROVER` environment variable to `network` and set the `NETWORK_PRIVATE_KEY` environment variable to your whitelisted private key.

3. Run with prover network:
   ```sh
   SP1_PROVER=network NETWORK_PRIVATE_KEY=... cargo run --release -p sudoku-script
   ```

## ZK Proof Technology

This project demonstrates several important concepts in zero-knowledge technology:

1. **Zero-Knowledge Proofs**: Allows a prover to convince a verifier that a statement is true without revealing any additional information.

2. **ZKVM (SP1)**: A special virtual machine that executes programs and produces proofs of their correct execution.

3. **EVM Compatibility**: The proofs can be verified on Ethereum and other EVM-compatible blockchains.

## Future Improvements

- Web UI for interactive Sudoku validation and visualization
- On-chain verification using the generated proofs
- Support for larger Sudoku variants (16x16, etc.)
- Optimizations for faster proof generation

## License

This project is available under the MIT License. See LICENSE-MIT for details.
