use verifier::Verifier;
use holder::Holder;

#[test]
fn primary() {
    // Verifier
    let mut verifier = Verifier::new();

    // Holder
    let mut holder = Holder::new();

    // Holder (which is the web application?) sends request to Verifier
    let holder_request = holder.request_create();

    // Verifier (which is the backend service?) receives request from Holder
    let Ok(verifier_response) = verifier.recv_request(holder_request) else {
        todo!();
    };

    // Holder receives response from Verifier
    let Ok(holder_request_2) = holder.recv_response(verifier_response) else {
        todo!();
    };

    // Verifier receives response from Holder
    let Ok(_verifier_response_2) = verifier.recv_request_2(holder_request_2) else {
        panic!("Holder has failed to prove ownership of private key");
    };

    // Assert result is correct
    // assert_eq!(verifier_response_2, Ok(()));

    // Log result
    println!("Holder has proven ownership of private key");
}