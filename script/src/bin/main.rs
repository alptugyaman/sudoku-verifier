//! An API server that verifies Sudoku puzzles using SP1 ZK proofs.
//!
//! To start the API server:
//! ```shell
//! RUST_LOG=info cargo run --release
//! ```

use axum::{
    extract::Json,
    http::Method,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sp1_sdk::{include_elf, SP1Stdin};
use std::net::SocketAddr;
use sudoku_lib::{verify_sudoku, SUDOKU_SIZE};
use tower_http::cors::{Any, CorsLayer};

/// The ELF file for the Sudoku verifier program.
pub const SUDOKU_ELF: &[u8] = include_elf!("sudoku-program");

/// JSON structure for Sudoku verification request.
#[derive(Deserialize, Debug)]
struct SudokuRequest {
    board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
    solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
}

/// JSON structure for Sudoku verification response.
#[derive(Serialize, Debug)]
struct SudokuResponse {
    is_valid: bool,
    proof_generated: bool,
    job_id: Option<String>,
    network_used: bool,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Setup the CORS layer for API
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // Setup the API routes
    let app = Router::new()
        .route("/validate-sudoku", post(validate_sudoku))
        .route("/", get(root_handler))
        .layer(cors);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Sudoku verifier API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// A simple GET handler for the home page
async fn root_handler() -> &'static str {
    "Sudoku ZK Verifier API\n\nUsage: Send a POST request to the /validate-sudoku endpoint"
}

/// API endpoint that validates a Sudoku board.
async fn validate_sudoku(Json(request): Json<SudokuRequest>) -> Json<SudokuResponse> {
    println!("Received sudoku verification request");

    // First, let's perform normal validation
    let is_valid = verify_sudoku(request.board, request.solution);
    println!("Sudoku verification result: {}", is_valid);

    // If it's valid, let's try to generate a ZK proof
    let mut proof_generated = false;
    let mut job_id: Option<String> = None;
    let mut network_used = false;

    if is_valid {
        println!("Attempting to generate ZK proof");

        // Check if we should use the network prover
        let use_network =
            std::env::var("SP1_PROVER").unwrap_or_else(|_| "local".to_string()) == "network";

        if use_network {
            println!("Using Succinct Prover Network for proof generation");
            match generate_network_proof(&request.board, &request.solution).await {
                Ok((_proof, job_id_str)) => {
                    proof_generated = true;
                    job_id = Some(job_id_str);
                    network_used = true;
                    println!("Successfully generated ZK proof via Succinct Network");
                }
                Err(err) => {
                    eprintln!("Error generating ZK proof via network: {}", err);
                }
            }
        } else {
            println!("Using local prover for proof generation");
            match generate_local_proof(&request.board, &request.solution) {
                Ok(_proof) => {
                    proof_generated = true;
                    println!("Successfully generated local ZK proof");
                }
                Err(err) => {
                    eprintln!("Error generating local ZK proof: {}", err);
                }
            }
        }
    }

    println!("Proof generation status: {}", proof_generated);

    Json(SudokuResponse {
        is_valid,
        proof_generated,
        job_id,
        network_used,
    })
}

/// Generates a ZK proof using Succinct Prover Network
async fn generate_network_proof(
    board: &[[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
    solution: &[[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
) -> Result<(Vec<u8>, String), String> {
    println!("Starting ZK proof generation with Succinct Prover Network");

    // Get private key from environment
    let private_key = std::env::var("NETWORK_PRIVATE_KEY")
        .map_err(|_| "Please set your NETWORK_PRIVATE_KEY environment variable".to_string())?;

    if private_key == "YOUR_PRIVATE_KEY_HERE" || private_key.is_empty() {
        return Err("Please set a valid NETWORK_PRIVATE_KEY environment variable".to_string());
    }

    // Create input using SP1Stdin
    let mut stdin = SP1Stdin::new();
    stdin.write(board);
    stdin.write(solution);

    println!("Prepared input for network proof generation");

    // Wrap the entire network proof generation in a panic-safe block
    let result = std::panic::catch_unwind(|| {
        // Generate the proof using the network prover
        let client = sp1_sdk::ProverClient::from_env();
        println!("Created ProverClient from env");

        let (pk, _vk) = client.setup(SUDOKU_ELF);
        println!("Generated proving and verifying keys for network");

        // Try to generate the proof
        match client.prove(&pk, &stdin).plonk().run() {
            Ok(proof) => {
                println!("Network proof generated successfully");
                Ok(proof)
            }
            Err(e) => {
                eprintln!("Error in network proof generation: {}", e);
                Err(format!("Failed to generate network proof: {}", e))
            }
        }
    });

    match result {
        Ok(Ok(proof)) => {
            // Generate a unique job ID for tracking
            let job_id = format!(
                "net_proof_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            println!("Network proof completed with job_id: {}", job_id);
            Ok((proof.bytes(), job_id))
        }
        Ok(Err(e)) => Err(e),
        Err(_) => {
            eprintln!("Network proof generation panicked!");
            Err("Network proof generation panicked - this usually indicates authentication or network issues".to_string())
        }
    }
}

/// Generates a ZK proof using local SP1 prover
fn generate_local_proof(
    board: &[[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
    solution: &[[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
) -> Result<Vec<u8>, String> {
    println!("Starting local ZK proof generation");

    // Create input using SP1Stdin
    let mut stdin = SP1Stdin::new();
    stdin.write(board);
    stdin.write(solution);

    println!("Prepared input for local proof generation");

    // Generate the proof using the local prover
    let client = sp1_sdk::ProverClient::from_env();
    let (pk, vk) = client.setup(SUDOKU_ELF);

    println!("Generated proving and verifying keys");

    let proof = client
        .prove(&pk, &stdin)
        .plonk()
        .run()
        .map_err(|e| format!("Failed to generate local proof: {}", e))?;

    println!("Local proof generated successfully");

    // Verify the proof
    client
        .verify(&proof, &vk)
        .map_err(|e| format!("Failed to verify local proof: {}", e))?;

    println!("Local proof verified successfully");

    Ok(proof.bytes())
}
