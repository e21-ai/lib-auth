//! # lib_auth
//!
//! A simple Rust library for generating and verifying Ed25519 signatures
//! on API keys or other messages. This wraps `ed25519-dalek` for easy
//! offline cryptographic license validation or secure messaging.

#[cfg(feature = "server")]
use rand::rngs::OsRng;
pub use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer,Verifier};

///  Signing key ~ Keypair, which includes both public and secret halves of an asymmetric key. 
#[cfg(feature = "server")]
pub fn generate_signing_key() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}

/// Get the public verifying key FROM a signing key
pub fn get_verifying_key(signing_key: &SigningKey) -> VerifyingKey {
    signing_key.verifying_key()
}

/// Sign a message using the private signing key
#[cfg(feature = "server")]
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Signature {
    signing_key.sign(message)
}

/// Verify a signature using the verifying (public) key
pub fn verify_with_public_key(
    public_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    public_key.verify(message, signature).is_ok()
}
