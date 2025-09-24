use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub id: String,
    pub from: String,
    pub to: String,
    pub value: u64,
    pub nonce: u64,
    pub prevs: Vec<String>,   // referências a tips
    pub blob_id: String,      // PoUW
    pub nonce_work: u64,      // nonce do PoUW
    pub pouw: String,         // resultado SHA3
    // pub sig_pqc: Vec<u8>,  // TODO: assinatura PQC
}

/// Peso cumulativo (placeholder)
pub fn cumulative_weight(_txid: &str) -> u64 {
    // TODO: somar referências diretas+indiretas
    1
}

/// Seleção de tips (placeholder)
pub fn select_random_tips(_k: usize) -> Vec<String> {
    vec![]
}