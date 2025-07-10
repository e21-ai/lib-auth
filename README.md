
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

```toml
lib-auth = { git = "ssh://github.com/e21-ai/lib-auth.git", features = ["server"] }

```

---

## 📚 Usage Example

### ✅ Sign and Verify API Key

Use this in your backend to:

* Generate Ed25519 signing keys
* Sign API keys or license tokens
* Verify messages

```rust
use lib_auth::{generate_signing_key, get_verifying_key, verify_with_public_key, Signature, Signer};

fn test_sign_and_verify_message() {
    // Generate a new signing key (private + public key)
    let keypair = generate_signing_key();

    // The message to be signed
    let message: &[u8] = b"This is a test of the tsunami alert system.";

    // Sign the message with the private key
    let signature: Signature = keypair.sign(message);

    // Verify using the full keypair directly (from ed25519-dalek)
    assert!(keypair.verify(message, &signature).is_ok());

    // Extract the public verifying key
    let public_key = get_verifying_key(&keypair);

    // Verify using your helper function with public key
    let is_valid = verify_with_public_key(&public_key, message, &signature);
    assert!(is_valid);

    println!("✅ Signature verified successfully!");
}

fn main(){
    test_sign_and_verify_message();
}

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
