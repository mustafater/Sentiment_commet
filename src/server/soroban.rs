use std::process::Command;
use log::{info, error};

/// Submit a negative comment to the Soroban smart contract.
/// This runs in a background blocking task to avoid blocking the Actix worker thread.
pub fn submit_negative_comment(comment_id: String, score: u32, content: String) {
    // Spawn a blocking task for the CLI command
    // We don't await the result (fire-and-forget)
    tokio::task::spawn_blocking(move || {
        let contract_id = std::env::var("CONTRACT_ID").unwrap_or_default();
        if contract_id.is_empty() {
            error!("CONTRACT_ID not set, skipping Soroban submission");
            return;
        }

        // Simple content "hash" (truncation) for demo purposes
        // In a real app, use SHA256
        let content_hash = if content.len() > 32 {
            content[..32].to_string() + "..."
        } else {
            content
        };

        info!("Submitting negative comment to Soroban: {} (score={}) - \"{}\"", comment_id, score, content_hash);

        // stellar contract invoke --id ... --network testnet --source deployer -- submit_negative ...
        let output = Command::new("stellar")
            .arg("contract")
            .arg("invoke")
            .arg("--id")
            .arg(&contract_id)
            .arg("--network")
            .arg("testnet")
            .arg("--source")
            .arg("deployer")
            .arg("--")
            .arg("submit_negative")
            .arg("--comment_id")
            .arg(&comment_id)
            .arg("--score")
            .arg(score.to_string())
            .arg("--content_hash")
            .arg(&content_hash)
            .output();

        match output {
            Ok(o) => {
                if o.status.success() {
                    info!("✅ Soroban submission successful for {}", comment_id);
                } else {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    error!("❌ Soroban submission failed: {}", stderr);
                }
            }
            Err(e) => {
                error!("❌ Failed to execute stellar CLI: {}", e);
            }
        }
    });
}
