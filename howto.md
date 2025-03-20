# How to Create a ZKVM Application with SP1

This guide walks you through the process of building a Zero-Knowledge Virtual Machine (ZKVM) application using SP1, based on our experience creating the Sudoku Verifier. Whether you're validating puzzle solutions, implementing confidential transactions, or developing any other privacy-preserving application, the steps below will help you get started.

## Introduction to SP1

SP1 is a Zero-Knowledge Virtual Machine developed by Succinct Labs. It enables developers to create zero-knowledge proofs for any computational statement written in Rust, allowing you to prove the correctness of a computation without revealing its inputs or intermediate state.

## Prerequisites

Before starting, ensure you have:

- **Rust Programming Environment:**
  - Rust installed via [rustup](https://rustup.rs/)
  - Rust nightly toolchain (`rustup toolchain install nightly`)
  - Basic familiarity with Rust

- **SP1 SDK:**
  - Follow the [installation guide](https://docs.succinct.xyz/getting-started/install.html)

- **Development Tools:**
  - A code editor (VS Code recommended with Rust extensions)
  - Git for version control

## Step 1: Project Setup

First, create a workspace structure for your application:

```bash
mkdir my-zk-app
cd my-zk-app
```

Create a workspace `Cargo.toml`:

```toml
[workspace]
members = [
    "lib",
    "program",
    "script"
]
```

This structure separates your application into three components:
- `lib`: Core business logic and data structures
- `program`: The program that runs inside the SP1 ZKVM
- `script`: CLI tools, API servers, or other interfaces

## Step 2: Implement the Core Library

The library contains the core validation logic that will be used by both your ZKVM program and external interfaces.

```bash
mkdir -p lib/src
```

Create `lib/Cargo.toml`:

```toml
[package]
name = "my-zk-lib"
version = "0.1.0"
edition = "2021"

[dependencies]
alloy-sol-types = "0.5.0"
```

Implement your core logic in `lib/src/lib.rs`:

```rust
use alloy_sol_types::sol;

// Define constants and types
pub const INPUT_SIZE: usize = 32;

// Define public values structure for proof verification
sol! {
    struct PublicValuesStruct {
        bool is_valid;
        // Add any other public values you need
    }
}

/// Core validation function
pub fn validate_input(input: [u8; INPUT_SIZE]) -> bool {
    // Implement your validation logic
    // For example, check if a sum is correct:
    let sum: u32 = input.iter().map(|&x| x as u32).sum();
    sum > 100
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation() {
        let valid_input = [10; INPUT_SIZE];
        assert!(validate_input(valid_input));
        
        let invalid_input = [1; INPUT_SIZE];
        assert!(!validate_input(invalid_input));
    }
}
```

## Step 3: Implement the ZKVM Program

Next, create the program that will run inside the SP1 virtual machine to generate proofs:

```bash
mkdir -p program/src
```

Create `program/Cargo.toml`:

```toml
[package]
name = "my-zk-program"
version = "0.1.0"
edition = "2021"

[dependencies]
sp1-zkvm = "0.1.0"
alloy-sol-types = "0.5.0"
my-zk-lib = { path = "../lib" }
```

Implement your program in `program/src/main.rs`:

```rust
//! A program that runs inside the SP1 ZKVM to generate proofs

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use my_zk_lib::{validate_input, PublicValuesStruct, INPUT_SIZE};

pub fn main() {
    // Read the input from the VM
    let input = sp1_zkvm::io::read::<[u8; INPUT_SIZE]>();
    
    // Perform validation
    let is_valid = validate_input(input);
    
    // Encode the result as public values
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { is_valid });
    
    // Commit the public values to the proof
    sp1_zkvm::io::commit_slice(&bytes);
}
```

## Step 4: Implement the Interface

Finally, create a CLI or API server to interface with your program:

```bash
mkdir -p script/src/bin
```

Create `script/Cargo.toml`:

```toml
[package]
name = "my-zk-script"
version = "0.1.0"
edition = "2021"

[dependencies]
sp1-sdk = "0.1.0"
bincode = "1.3.3"
serde = { version = "1.0", features = ["derive"] }
clap = { version = "4.1", features = ["derive"] }
my-zk-lib = { path = "../lib" }
```

Implement a CLI tool in `script/src/bin/main.rs`:

```rust
//! CLI tool for generating and verifying proofs

use clap::{Parser, Subcommand};
use my_zk_lib::{INPUT_SIZE, validate_input};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::path::PathBuf;
use std::fs;

// Include the ELF binary of your program
pub const PROGRAM_ELF: &[u8] = include_elf!("my-zk-program");

#[derive(Parser)]
#[command(version, about = "Zero-Knowledge Proof Generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a proof for the given input
    Prove {
        /// Input file path (should contain INPUT_SIZE bytes)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file path for the proof
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Verify a proof
    Verify {
        /// Proof file path
        #[arg(short, long)]
        proof: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Prove { input, output } => {
            // Read input file
            let input_data = fs::read(&input)?;
            if input_data.len() != INPUT_SIZE {
                return Err(format!("Input file must contain exactly {} bytes", INPUT_SIZE).into());
            }
            
            // Convert to expected input type
            let mut input_array = [0u8; INPUT_SIZE];
            input_array.copy_from_slice(&input_data);
            
            // Generate proof
            println!("Generating proof...");
            let proof = generate_zk_proof(input_array)?;
            
            // Write proof to file
            fs::write(output, proof)?;
            println!("Proof saved successfully!");
        },
        Commands::Verify { proof } => {
            println!("Verifying proof...");
            // Read and verify the proof
            let proof_data = fs::read(proof)?;
            // Implement verification logic
            println!("Proof is valid!");
        }
    }
    
    Ok(())
}

/// Generate a ZK proof for the given input
fn generate_zk_proof(input: [u8; INPUT_SIZE]) -> Result<Vec<u8>, String> {
    // Create SP1 input
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);
    
    // Set up the prover
    let client = sp1_sdk::ProverClient::from_env();
    println!("Setting up proving key...");
    let (pk, vk) = client.setup(PROGRAM_ELF);
    println!("Successfully set up proving key");
    
    // Generate proof
    println!("Generating proof...");
    match client.prove(&pk, &stdin).run() {
        Ok(proof_with_values) => {
            println!("Successfully generated ZK proof");
            // Serialize the proof
            let serialized_proof = bincode::serialize(&proof_with_values)
                .map_err(|e| format!("Failed to serialize proof: {}", e))?;
            println!("Serialized ZK proof of size {} bytes", serialized_proof.len());
            Ok(serialized_proof)
        },
        Err(err) => {
            eprintln!("Failed to generate proof: {}", err);
            Err(format!("Failed to generate proof: {}", err))
        }
    }
}
```

## Step 5: Building and Running

To build your program for the SP1 ZKVM:

```bash
cd program
cargo prove build
```

This will generate an ELF binary that can be used by the SP1 virtual machine.

To run your CLI tool:

```bash
cd ../script
cargo run -- prove --input /path/to/input.bin --output /path/to/proof.bin
```

## Step 6: Creating an API Server (Optional)

If you want to expose your application as a web service, you can implement an API server using a framework like Axum or Actix Web.

In `script/Cargo.toml`, add:

```toml
[dependencies]
axum = "0.6.0"
tokio = { version = "1.0", features = ["full"] }
```

Create `script/src/bin/server.rs`:

```rust
//! API server for generating and verifying proofs

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use my_zk_lib::{INPUT_SIZE, validate_input};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::net::SocketAddr;

// Include the ELF binary of your program
pub const PROGRAM_ELF: &[u8] = include_elf!("my-zk-program");

#[derive(Deserialize)]
struct ProofRequest {
    input: Vec<u8>,
}

#[derive(Serialize)]
struct ProofResponse {
    is_valid: bool,
    proof_generated: bool,
    proof: Option<String>, // Base64 encoded proof
}

#[tokio::main]
async fn main() {
    // Build the router
    let app = Router::new()
        .route("/", get(|| async { "ZK Proof API" }))
        .route("/generate-proof", post(generate_proof));

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate_proof(Json(request): Json<ProofRequest>) -> Result<Json<ProofResponse>, StatusCode> {
    if request.input.len() != INPUT_SIZE {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Convert to expected input type
    let mut input_array = [0u8; INPUT_SIZE];
    input_array.copy_from_slice(&request.input);
    
    // Validate the input (optional, for immediate feedback)
    let is_valid = validate_input(input_array);
    
    // Generate proof if input is valid
    let mut proof_generated = false;
    let mut proof = None;
    
    if is_valid {
        match generate_zk_proof(input_array) {
            Ok(proof_data) => {
                proof_generated = true;
                // Convert proof to base64 for JSON transmission
                proof = Some(base64::encode(&proof_data));
            },
            Err(_) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
    
    Ok(Json(ProofResponse {
        is_valid,
        proof_generated,
        proof,
    }))
}

// Reuse the generate_zk_proof function from earlier
```

## Step 7: Advanced Topics

### EVM Compatibility

For Ethereum/EVM-compatible proofs, you can use Groth16 or PLONK proving systems:

```bash
# Create a script for generating EVM-compatible proofs
touch script/src/bin/evm.rs
```

Implementation details depend on your specific requirements.

### Prover Network

For production use, utilize the Succinct prover network:

1. Create an `.env` file:
```
SP1_PROVER=network
NETWORK_PRIVATE_KEY=your_private_key
```

2. Update your code to respect these environment settings.

### Zero-Knowledge Design Patterns

When designing your ZK application, consider:

1. **Minimize Circuit Size**: Keep computations simple within the ZK context
2. **Public vs. Private Inputs**: Carefully decide what should be kept private
3. **Modular Design**: Break complex logic into smaller, composable circuits
4. **Optimization**: Use SP1-specific optimizations for complex computations

## Conclusion

Building a ZKVM application with SP1 involves several components working together:

1. A core library with your validation logic
2. A program that runs inside the SP1 ZKVM
3. Interface components for generating and verifying proofs

With this architecture, you can create powerful zero-knowledge applications that preserve privacy while proving computational integrity.

For more advanced use cases or optimizations, refer to the [SP1 documentation](https://docs.succinct.xyz/) and community resources.

## Troubleshooting Tips

- **Memory Issues**: For large proofs (e.g., EVM-compatible ones), ensure you have at least 128GB of RAM
- **Build Errors**: Check that you're using the correct Rust toolchain (nightly may be required)
- **Proof Generation Failures**: Start with simpler programs and gradually increase complexity
- **Performance Issues**: Consider using the Succinct prover network for better performance 