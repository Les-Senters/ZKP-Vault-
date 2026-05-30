mod compliance_prover;
mod silo_verifier;

fn main() {
    println!("--- Project ZKP-Vault Initialized ---");

    // 1. Simulate the User's Local Device Environment
    let local_fingerprint = compliance_prover::DeviceFingerprint {
        secure_enclave_signature: "Hardware_Signature_Verified_True".to_string(),
        cluster_id: vec![10, 20, 30, 40],
    };

    let user_state = compliance_prover::ComplianceState {
        is_premium_subscriber: true,
        real_time_policy_version: 2026,
        storage_compartment_hash: "Silo_Bucket_99A".to_string(),
    };

    // 2. Generate the local ZKP compliance token
    println!("Generating secure ZKP on-device...");
    let generated_proof = compliance_prover::generate_compliance_proof(&local_fingerprint, &user_state)
        .expect("Failed to generate compliance token.");

    // 3. Simulate sending only the token to the Cloud Silo
    println!("\nSending token to cloud silo gateway...");
    let cloud_controller = silo_verifier::CloudSiloController {
        database_id: "Enterprise_Cloud_Vault_Beta".to_string(),
    };

    match cloud_controller.verify_and_fetch_context(&generated_proof) {
        Ok(context) => {
            println!("\n[SUCCESS] Cloud verified the math token flawlessly!");
            println!("AI Core granted temporary access to data segment: {}", context);
            println!("Result: Flawless cross-device memory achieved with zero data exposure.");
        },
        Err(e) => println!("[ERROR] Access Denied: {}", e),
    }
}
