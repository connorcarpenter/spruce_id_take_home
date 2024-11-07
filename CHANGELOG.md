# Changelog

### 11/6/2024 10:45 PM MST
- Added expiration time to Verifier's stored nonce
- Verifier stores multiple nonces per user, one per challenge request
- Added integration test to ensure multiple active nonces per user work as expected

### 11/6/2024 9:37 PM MST
- Turned integration test into a runnable binary, mostly to facilitate logging
- Added detailed logging to the integration test

### 11/6/2024 8:25 PM MST
- Constraining Nonce, PublicKey, and Signature to fixed-length byte arrays

### 11/3/2024 4:56 PM MST
- Pulling in ring and uuid crates, stubbing out data
- Defined necessary data in Requests/Responses
- Implemented methods in Verifier and Holder
- Happy path test works end to end!
- Done for today, will add docs and more tests and logging tomorrow/soon!

### 11/3/2024 4:07 PM MST
- Added Registration step to Holder/Verifier communication, UserId abstracts away the public key, preferably there would be additional auth steps that happen here!
- Naming things is hard :) Renamed various structs, errors, and methods to make example more readable
- Pulled in `thiserror` and `anyhow` crates to reduce boilerplate
- Test bubbles up errors from process execution, handles them, reducing boilerplate

### 11/3/2024 3:05 PM MST
- Stubbed out verifier & holder crates, types, define API
- Stubbed out shared types into separate crate
- Created running integration test that calls the appropriate methods to simulate the request/response flow of the Holder and Verifier
- Added rustfmt config

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