//! An API server that verifies Sudoku puzzles using SP1 ZK proofs.
//!
//! API sunucusunu başlatmak için:
//! ```shell
//! RUST_LOG=info cargo run --release
//! ```

use alloy_sol_types::SolType;
use axum::{
    extract::Json,
    http::Method,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use clap::Parser;
use serde::{Deserialize, Serialize};
use sudoku_lib::{verify_sudoku, PublicValuesStruct, SUDOKU_SIZE};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use tower_http::cors::{Any, CorsLayer};
use bincode;
use chrono;

/// The ELF file for the Sudoku verifier program.
pub const SUDOKU_ELF: &[u8] = include_elf!("sudoku-program");

/// API için komut satırı argümanları.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "3000")]
    port: u16,

    #[clap(long, default_value = "true")]
    use_zkp: bool,
}

/// Sudoku doğrulama isteği için JSON yapısı.
#[derive(Deserialize, Debug)]
struct SudokuRequest {
    board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
    solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
}

/// Sudoku doğrulama yanıtı için JSON yapısı.
#[derive(Serialize, Debug)]
struct SudokuResponse {
    is_valid: bool,
    proof_generated: bool,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Parse the command line arguments.
    let args = Args::parse();

    // Setup the CORS layer for API
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // Setup the API routes
    let app = Router::new()
        .route("/validate-sudoku", post(validate_sudoku))
        .route("/", get(root_handler))  // Ana sayfa için basit bir GET handler'ı ekleyelim
        .layer(cors);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    println!("Sudoku verifier API server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Ana sayfa için basit bir GET handler'ı
async fn root_handler() -> &'static str {
    "Sudoku ZK Doğrulayıcı API\n\nKullanım: /validate-sudoku endpoint'ine POST isteği gönderin"
}

/// Sudoku tahtasını doğrulayan API endpoint'i.
async fn validate_sudoku(Json(request): Json<SudokuRequest>) -> Json<SudokuResponse> {
    println!("Received sudoku verification request");

    // Önce normal doğrulama yapalım
    let is_valid = verify_sudoku(request.board, request.solution);
    println!("Sudoku verification result: {}", is_valid);

    // Eğer geçerliyse ZK kanıtı oluşturmaya çalışalım
    let mut proof_generated = false;
    
    if is_valid {
        println!("Attempting to generate ZK proof");
        
        // SP1 ile kanıt oluşturmayı dene
        match generate_zk_proof(&request.board, &request.solution) {
            Ok(proof) => {
                proof_generated = true;
                println!("Successfully generated ZK proof of size {} bytes", proof.len());
                
                // ZK proof'u dosyaya kaydet
                let proof_dir = "proofs";
                std::fs::create_dir_all(proof_dir).unwrap_or_else(|_| {
                    println!("Proofs directory already exists or cannot be created");
                });
                
                let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                let proof_path = format!("{}/zk_proof_{}.bin", proof_dir, timestamp);
                
                match std::fs::write(&proof_path, &proof) {
                    Ok(_) => println!("ZK proof saved to file: {}", proof_path),
                    Err(e) => eprintln!("Failed to save ZK proof to file: {}", e)
                }
            },
            Err(err) => {
                eprintln!("Error generating ZK proof: {}", err);
            }
        }
    }
    
    println!("Proof generation status: {}", proof_generated);
    
    Json(SudokuResponse {
        is_valid,
        proof_generated,
    })
}

/// SP1 ile ZK kanıtı oluşturur.
fn generate_zk_proof(board: &[[u8; SUDOKU_SIZE]; SUDOKU_SIZE], solution: &[[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> Result<Vec<u8>, String> {
    println!("Starting ZK proof generation process");
    
    // SP1Stdin kullanarak girdi oluşturma
    let mut stdin = SP1Stdin::new();
    
    // Board ve çözümü stdin'e yazma
    stdin.write(board);
    stdin.write(solution);
    
    println!("");
    
    // SP1 prover client'ını başlat
    let client = sp1_sdk::ProverClient::from_env();
    println!("Successfully created SP1 ProverClient");
    
    // SP1 proof oluştur (setup artık Result değil doğrudan tuple döndürüyor)
    println!("Setting up proving key...");
    let (pk, vk) = client.setup(SUDOKU_ELF);
    println!("Successfully set up proving key and verification key");
    
    println!("Starting proof generation with SP1...");
    match client.prove(&pk, &stdin).run() {
        Ok(proof_with_values) => {
            println!("Successfully generated ZK proof");
            // Proof'u vektöre serileştirmek için
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
