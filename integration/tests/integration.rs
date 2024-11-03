
use anyhow::Result;

use verifier::Verifier;
use holder::Holder;

#[test]
fn primary() {
    // Verifier
    let mut verifier = Verifier::new();

    // Holder (which is the web application)
    let mut holder = Holder::new();

    // Execute Process Test
    let process_result = execute_process(&mut holder, &mut verifier);

    if let Err(err) = process_result {

        let backtrace = err.backtrace();

        // Print error message
        eprintln!("Error occurred during test execution: {}", err);
        eprintln!("Backtrace: {:?}", backtrace);

        // Panic
        panic!("Test failed due to error.");
    }

    assert!(process_result.is_ok());
}

fn execute_process(holder: &mut Holder, verifier: &mut Verifier) -> Result<()> {

    // Holder registers with Verifier
    let holder_register_request = holder.create_register_request();

    // Verifier receives register request from Holder
    let verifier_register_response = verifier.recv_register_request(holder_register_request)?;

    // Holder receives register response from Verifier
    holder.recv_register_response(verifier_register_response)?;

    // Holder sends challenge request to Verifier
    let holder_challenge_request = holder.create_challenge_request()?;

    // Verifier receives challenge request from Holder
    let verifier_challenge_response = verifier.recv_challenge_request(holder_challenge_request)?;

    // Holder receives challenge response from Verifier
    let holder_verify_request = holder.recv_challenge_response(verifier_challenge_response)?;

    // Verifier receives verify request from Holder
    let _verifier_verify_response = verifier.recv_verify_request(holder_verify_request)?;

    return Ok(());
}