[![Rust](https://github.com/e21-ai/lib-auth/actions/workflows/rust.yml/badge.svg)](https://github.com/e21-ai/lib-auth/actions/workflows/rust.yml)

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
lib-auth = { git = "git://github.com/e21-ai/lib-auth.git" }
````

This includes only signature **verification** logic, safe for offline use in:

* Tauri apps
* CLI utilities
* Browsers (via WASM)

> ℹ️ The `client` feature is enabled by default.

---

### 🔐 Server (Sign + Verify)

```toml
lib-auth = { git = "git://github.com/e21-ai/lib-auth.git", features = ["server"] }

```

Use this in your backend to:

* Generate Ed25519 signing keys
* Sign API keys or license tokens
* Verify messages

---

## 📚 Usage Examples

### ✅ Verifying API Key (Client-Side)

```rust
use lib_auth::verify_with_public_key;
use ed25519_dalek::{VerifyingKey, Signature};
use base64::{engine::general_purpose, Engine};

let public_key_b64 = "...";  // base64-encoded public key
let signature_b64 = "...";   // base64-encoded signature from server
let message = "user:alice@example.com:xyz123"; // original signed message

let pubkey_bytes = general_purpose::STANDARD.decode(public_key_b64).unwrap();
let signature_bytes = general_purpose::STANDARD.decode(signature_b64).unwrap();

let verifying_key = VerifyingKey::from_bytes(&pubkey_bytes).unwrap();
let signature = Signature::from_bytes(&signature_bytes).unwrap();

let valid = verify_with_public_key(&verifying_key, message.as_bytes(), &signature);

assert!(valid);
```

---

### 🔐 Signing API Key (Server-Side)

```rust
#[cfg(feature = "server")]
use lib_auth::{generate_signing_key, get_verifying_key, sign_message};
use base64::{engine::general_purpose, Engine};

#[cfg(feature = "server")]
fn generate_key_and_signature() {
    let signing_key = generate_signing_key();
    let verifying_key = get_verifying_key(&signing_key);

    let message = "user:alice@example.com:xyz123";
    let signature = sign_message(&signing_key, message.as_bytes());

    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
    let pubkey_b64 = general_purpose::STANDARD.encode(verifying_key.to_bytes());

    println!("key: {}", message);
    println!("signature: {}", signature_b64);
    println!("public_key: {}", pubkey_b64);
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
