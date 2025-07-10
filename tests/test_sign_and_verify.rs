#[cfg(test)]
mod tests {

    use lib_auth::verify_with_public_key;
    use lib_auth::get_verifying_key;
    use ed25519_dalek::Signer;
    use ed25519_dalek::Signature;
    use lib_auth::generate_signing_key;


    #[test]
    fn test_sign_and_verify_message() {
        // Generate a new keypair
        let keypair = generate_signing_key();

        // Define the message
        let message: &[u8] = b"This is a test of the tsunami alert system.";

        // Sign the message
        let signature: Signature = keypair.sign(message);

        // Verify using the keypair directly (via dalek's Verifier)
        assert!(keypair.verify(message, &signature).is_ok());

        // Also verify using the public key + your helper function
        let public_key = get_verifying_key(&keypair);
        let is_valid = verify_with_public_key(&public_key, message, &signature);
        assert!(is_valid);
    }
}
