use verifier::Verifier;
use holder::Holder;

#[test]
fn primary() {
    // Verifier
    let mut verifier = Verifier::new();

    // Holder
    let mut holder = Holder::new();

    // Holder (which is the web application?) sends request to Verifier
    let holder_challenge_request = holder.create_challenge_request();

    // Verifier (which is the backend service?) receives request from Holder
    let Ok(verifier_challenge_response) = verifier.recv_challenge_request(holder_challenge_request) else {
        todo!();
    };

    // Holder receives response from Verifier
    let Ok(holder_verify_request) = holder.recv_challenge_response(verifier_challenge_response) else {
        todo!();
    };

    // Verifier receives response from Holder
    let Ok(verifier_verify_response) = verifier.recv_verify_request(holder_verify_request) else {
        panic!("Holder has failed to prove ownership of private key");
    };

    // Assert result is correct
    // assert_eq!(verifier_response_2, Ok(()));

    // Log result
    println!("Holder has proven ownership of private key");
}