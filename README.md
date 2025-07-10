
A lightweight Rust library for generating and verifying Ed25519 signatures on API keys or arbitrary messages.  
Designed for secure license validation, stateless API keys, and offline verification in client apps (e.g., Tauri).

Built on top of [`ed25519-dalek`](https://docs.rs/ed25519-dalek), with clean wrappers for signing and verifying messages.

---

## ✨ Features

- 🔐 Ed25519 keypair generation
- ✍️ Message signing with private key
- ✅ Signature verification using public key
- 🛡️ Offline validation in Tauri or CLI apps
- ⚙️ Feature-gated API (`client` vs `server`)

---

## 🔧 Installation

In your own `Cargo.toml`, choose a feature based on where you're using `lib_auth`:

### ✅ Client App (Verify Only)

```toml
lib-auth = { git = "ssh://github.com/e21-ai/lib-auth.git" }
````

This includes only signature **verification** logic, safe for offline use in:

* Tauri apps
* CLI utilities
* Browsers (via WASM)

> ℹ️ The `client` feature is enabled by default.

---

### 🔐 Server (Sign + Verify)

```toml
lib-auth = { git = "ssh://github.com/e21-ai/lib-auth.git", features = ["server"] }

```

Use this in your backend to:

* Generate Ed25519 signing keys
* Sign API keys or license tokens
* Verify messages

---

## 📚 Usage Examples

```
[dependencies]
base64 = "0.22.1"
ed25519-dalek = "2.2.0"
lib-auth = { git = "ssh://github.com/e21-ai/lib-auth.git" }
rand = "0.9.1"
```

### ✅ Verifying API Key (Client-Side)

```rust
use lib_auth::generate_signing_key;

fn main() {
    // Generate two signing keys
    let key1 = generate_signing_key();
    let key2 = generate_signing_key();

    // Convert keys to byte arrays
    let bytes1 = key1.to_bytes();
    let bytes2 = key2.to_bytes();

    // Check keys are different (extremely unlikely to be the same)
    assert_ne!(bytes1, bytes2, "Two generated keys should not be identical");

    // Check keys are 32 bytes long
    assert_eq!(bytes1.len(), 32, "Key1 length should be 32 bytes");
    assert_eq!(bytes2.len(), 32, "Key2 length should be 32 bytes");

    println!("Generated two distinct 32-byte signing keys successfully.");
}

```

---

### 🔐 Signing API Key (Server-Side)

```rust
use lib_auth::{Signer, generate_signing_key, get_verifying_key, verify_with_public_key};

fn main() {
    // Generate a new Ed25519 signing key (includes private and public keys)
    let signing_key = generate_signing_key();

    // The message to sign
    let message = b"This is a test of the tsunami alert system.";

    // Sign the message with the private key
    let signature = signing_key.sign(message);

    // Verify signature using the public key extracted from signing key
    let public_key = get_verifying_key(&signing_key);
    let is_valid = verify_with_public_key(&public_key, message, &signature);

    assert!(is_valid, "Signature should be valid");

    println!("Signature verification succeeded!");
}

```
```
Signature verification succeeded!
```
---

## 🔐 Feature Flags

| Feature     | Description                        | Private Key Access | Client Safe |
| ----------- | ---------------------------------- | ------------------ | ----------- |
| `client`    | Enable signature verification only | ❌ No               | ✅ Yes       |
| `server`    | Enable keygen + signing + verify   | ✅ Yes              | 🚫 No       |
| *(default)* | Same as `client`                   | ❌ No               | ✅ Yes       |

> 💡 You can disable default features with:
>
> ```toml
> lib-auth = { git = "...", default-features = false, features = ["server"] }
> ```

---

## 🧪 Running Tests

```bash
cargo test --features server
```

Includes a test for roundtrip signing and verification.

---

## 📦 Example Use Cases

* License validation in offline desktop apps
* API key signing for stateless auth
* Cryptographic proofs with no server-side storage

---

## 👤 Author

Built with 🦀 by \[e21]

```
Happy signing 🖋️
```
