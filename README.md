# spruce_id_take_home
A take home interview assignment given by the wonderful https://spruceid.com/about .
Implemented by Connor Carpenter (https://github.com/connorcarpenter)

## Assignment Instructions Given:
The goal of this assignment is to implement a mechanism to prove ownership of a private key. It should be done in Rust.

### Details:
- Two actors are involved: a holder (e.g. a web application), and a verifier (e.g. a backend service).
- The holder should sign a payload with the private key.
- The verifier should verify the payload and signature to establish that the holder controls the private key.
- A nonce should be used to prevent replay of attestations.

### Advice:
- A complete setup with a frontend and a backend is not expected. You can simply write a unit test demonstrating the interactions between a holder type and a verifier type.
- If you are unable to implement a complete solution, write down your thoughts and explain the limitations of your solution.
- This assignment is fairly simple, and its purpose is mostly to show familiarity with Rust and its ecosystem, and a basic understanding of public key cryptography.

This assignment should not take more than 4 hours. At the end you should send a link to a repo or a tarball of it, with a README with some instructions on how to run the demo.

### How to Run:
- Clone this repository
- Navigate into the repository directory
- Run ```cargo test --package tests```
- TODO: more logging while testing to better track what is happening

