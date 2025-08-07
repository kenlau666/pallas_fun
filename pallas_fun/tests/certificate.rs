// use pallas_fun::credential::StakeCredentialWrapper;
// use pallas_fun::wrapper::{CertificateKind, CertificateWrapper};

// #[test]
// fn test_certificate_wrapper_encode_decode() {
//     // Example: StakeRegistration certificate
//     let stake_credential =
//         StakeCredentialWrapper::new(pallas_fun::credential::StakeCredentialKind::AddrKeyhash(
//             "276fd18711931e2c0e21430192dbeac0e458093cd9d1fcd7210f64b3".to_string(),
//         ))
//         .expect("valid stake credential");

//     let cert_kind = CertificateKind::StakeRegistration {
//         stake_credential_wrapper: stake_credential,
//     };

//     let wrapper = CertificateWrapper::new(cert_kind).expect("should create certificate wrapper");

//     let encoded = wrapper.encode();
//     let decoded = CertificateWrapper::decode(encoded).expect("should decode");

//     assert_eq!(wrapper, decoded);
// }

// #[test]
// #[should_panic]
// fn test_certificate_wrapper_invalid_keyhash() {
//     // Example: StakeRegistration with invalid keyhash
//     let stake_credential = StakeCredentialWrapper::new(
//         pallas_fun::credential::StakeCredentialKind::AddrKeyhash("invalid".to_string()),
//     );

//     assert!(
//         stake_credential.is_err(),
//         "should fail with invalid keyhash"
//     );

//     let cert_kind = CertificateKind::StakeRegistration {
//         stake_credential_wrapper: stake_credential.unwrap(),
//     };

//     let wrapper = CertificateWrapper::new(cert_kind);
//     assert!(
//         wrapper.is_err(),
//         "should fail to create certificate wrapper with invalid keyhash"
//     );
// }
