# Changelog

### 11/3/2024 2:21 PM MST
- Setting up new repository for local development
- Adding changelog to allow SpruceId to follow along with my thought process
- Adding instructions to README.md
- Initial implementation brainstorm includes:
  - Separating out Holder and Verifier into separate crates, don't want to share any unnecessary code
  - Holder and Verifier should share a common crate for shared types
  - Will have a test suite that pulls in each crate
  - Still need to research appropriate mature crates for public/private key cryptography
  - Initial thoughts are to use `ring` , `ed25519-dalek` for crypto methods
  - Perhaps should use `anyhow` for error handling? (or `thiserror` for component crates)
  - Should we make this async? Not really part of the assignment, but I'd like to use async Rust by default.. I would use `tokio` if writing this code for Spruce
  - Keep eyes open for any gotchas here .. will need to research to make sure I'm following best practices for this!

- Implementation ideas:
```
// Verifier
let verifier = Verifier::new();

// Holder
let holder = Holder::new();

// Verifier creates nonce
let nonce = verifier.create_nonce();

// Holder signs nonce
let holder_response = holder.recv_none(nonce);

// Verifier verifies result
let verifier_response = verifier.verify_response(nonce, holder_response);

// Log result
match verifier_response {
    Ok(_) => println!("Holder has proven ownership of private key"),
    Err(_) => println!("Holder has failed to prove ownership of private key"),
}

// Assert result is correct
assert_eq!(verifier_response, Ok(()));
```