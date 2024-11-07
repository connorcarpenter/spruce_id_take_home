use anyhow::Result;

use holder::Holder;
use verifier::Verifier;

#[test]
fn can_verify_multiple_times_per_user_simultaneously() {
    // Verifier (which is some backend service)
    let mut verifier = Verifier::new();

    // Holder (which is the web application)
    let mut holder = Holder::new();

    // Execute Process Test
    let test_result = run_successful_test(&mut holder, &mut verifier);

    if let Err(err) = test_result {
        let backtrace = err.backtrace();

        // Print error message
        eprintln!("Error occurred during test execution: {}", err);
        eprintln!("Backtrace: {:?}", backtrace);

        // Panic
        panic!("Test failed due to error.");
    }

    assert!(test_result.is_ok());
}

fn run_successful_test(holder: &mut Holder, verifier: &mut Verifier) -> Result<()> {
    // Registration
    let holder_register_request = holder.create_register_request();
    let verifier_register_response = verifier.recv_register_request(holder_register_request)?;

    holder.recv_register_response(verifier_register_response)?;

    // Holder sends challenge request to Verifier, receives response, creates verification request
    let holder_challenge_request = holder.create_challenge_request()?;
    let verifier_challenge_response_1 =
        verifier.recv_challenge_request(holder_challenge_request)?;
    let holder_verify_request_1 = holder.recv_challenge_response(verifier_challenge_response_1)?;

    // Holder does the same thing again
    let verifier_challenge_response_2 =
        verifier.recv_challenge_request(holder_challenge_request)?;
    let holder_verify_request_2 = holder.recv_challenge_response(verifier_challenge_response_2)?;

    // Holder sends verify requests to Verifier
    let verifier_verify_response_1 = verifier.recv_verify_request(holder_verify_request_1)?;
    let verifier_verify_response_2 = verifier.recv_verify_request(holder_verify_request_2)?;

    // Holder receives verify responses from Verifier
    holder.recv_verify_response(verifier_verify_response_1)?;
    holder.recv_verify_response(verifier_verify_response_2)?;

    Ok(())
}
