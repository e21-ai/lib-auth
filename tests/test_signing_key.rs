#[cfg(test)]
mod tests {
    use super::*;
    use lib_auth::generate_signing_key;
    
    #[test]
    fn test_generate_signing_key() {
        let key1 = generate_signing_key();
        let key2 = generate_signing_key();
        
        // Keys should be different (extremely unlikely to be the same)
        assert_ne!(key1.to_bytes(), key2.to_bytes());
        
        // Keys should be 32 bytes long
        assert_eq!(key1.to_bytes().len(), 32);
        assert_eq!(key2.to_bytes().len(), 32);
    }
}