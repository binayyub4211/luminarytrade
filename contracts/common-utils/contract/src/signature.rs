// contracts/common-utils/src/signature.rs

use soroban_sdk::{Env, BytesN, Vec};

pub fn verify_signature(
    env: &Env,
    public_key: BytesN<32>,
    message: Vec<u8>,
    signature: BytesN<64>,
) -> bool {
    env.crypto().ed25519_verify(&public_key, &message, &signature)
}