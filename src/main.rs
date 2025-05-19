use axum::{routing::post, Router, Json, extract::State};
use clap::{Parser, Subcommand};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the IdP server
    Server {
        /// Path to accounts file
        #[arg(long, default_value = "accounts.json")]
        accounts: PathBuf,
        /// Address to bind
        #[arg(long, default_value = "127.0.0.1:3000")]
        addr: String,
    },
    /// Create a new service account
    CreateAccount {
        /// Path to accounts file
        #[arg(long, default_value = "accounts.json")]
        accounts: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    client_id: String,
    client_secret: String,
}

type Accounts = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Server { accounts, addr } => {
            let accounts_map = load_accounts(&accounts);
            let state = Accounts::new(Mutex::new(accounts_map));
            let app = Router::new().route("/token", post(issue_token)).with_state(state);
            println!("Listening on {}", addr);
            axum::Server::bind(&addr.parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Commands::CreateAccount { accounts } => {
            let mut accounts_map = load_accounts(&accounts);
            let id = Uuid::new_v4().to_string();
            let secret = Uuid::new_v4().to_string();
            accounts_map.insert(id.clone(), secret.clone());
            save_accounts(&accounts, &accounts_map);
            println!("Created account: {} {}", id, secret);
        }
    }
}

#[derive(Deserialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

async fn issue_token(State(accounts): State<Accounts>, Json(req): Json<TokenRequest>) -> Json<TokenResponse> {
    let map = accounts.lock().await;
    match map.get(&req.client_id) {
        Some(secret) if secret == &req.client_secret => {
            let claims = serde_json::json!({
                "sub": req.client_id,
                "exp": (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
            });
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret"))
                .unwrap();
            Json(TokenResponse { token })
        }
        _ => Json(TokenResponse { token: "".into() }),
    }
}

fn load_accounts(path: &PathBuf) -> HashMap<String, String> {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_accounts(path: &PathBuf, map: &HashMap<String, String>) {
    let data = serde_json::to_string_pretty(map).unwrap();
    fs::write(path, data).unwrap();
}

