use crate::routing::psp::PSP;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: Uuid,
    pub min_amount: u64,
    pub max_amount: u64,
    pub currency: String,
    pub psp: PSP,
}