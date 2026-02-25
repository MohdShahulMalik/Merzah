use base64::{engine::general_purpose, Engine as _};
use rand::{thread_rng, Rng};

use crate::errors::oauth::{StateError, StateResult};

pub fn generate_state() -> StateResult<String> {
    let mut bytes = [0u8; 32];
    thread_rng().fill(&mut bytes);
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    if encoded.is_empty() {
        return Err(StateError::GenerationError);
    }
    Ok(encoded)
}

pub fn validate_state(state: &str, stored_state: &str) -> bool {
    !state.is_empty() && !stored_state.is_empty() && state == stored_state
}
