use anyhow::Result;
use log::{info, warn};

use holder::Holder;
use verifier::Verifier;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("");
    info!("Starting Integration Test.");
    info!("");

    // Verifier (which is some backend service)
    let mut verifier = Verifier::new();

    // Holder (which is the web application)
    let mut holder = Holder::new();

    // Execute Process Test
    let test_result = run_successful_test(&mut holder, &mut verifier);

    if let Err(err) = test_result {
        let backtrace = err.backtrace();

        // Print error message
        warn!("Error occurred during test execution: {}", err);
        warn!("Backtrace: {:?}", backtrace);

        // Panic
        panic!("Test failed due to error.");
    }

    assert!(test_result.is_ok());
}

fn log_request(request_name: &str) {
    info!("");
    info!("Holder -> [{}] -> Verifier", request_name);
    info!("");
}

fn log_response(response_name: &str) {
    info!("");
    info!("Holder <- [{}] <- Verifier", response_name);
    info!("");
}

fn run_successful_test(holder: &mut Holder, verifier: &mut Verifier) -> Result<()> {
    // Holder sends register request to Verifier
    let holder_register_request = holder.create_register_request();
    log_request("HolderRegisterRequest");
    let verifier_register_response = verifier.recv_register_request(holder_register_request)?;

    // Holder receives register response from Verifier
    log_response("HolderRegisterResponse");
    holder.recv_register_response(verifier_register_response)?;

    // Holder sends challenge request to Verifier
    let holder_challenge_request = holder.create_challenge_request()?;
    log_request("HolderChallengeRequest");
    let verifier_challenge_response = verifier.recv_challenge_request(holder_challenge_request)?;

    // Holder receives challenge response from Verifier
    log_response("HolderChallengeResponse");
    let holder_verify_request = holder.recv_challenge_response(verifier_challenge_response)?;

    // Holder sends verify request to Verifier
    log_request("HolderVerifyRequest");
    let verifier_verify_response = verifier.recv_verify_request(holder_verify_request)?;

    // Holder receives verify response from Verifier
    log_response("HolderVerifyResponse");
    return holder
        .recv_verify_response(verifier_verify_response)
        .map_err(|e| e.into());
}
