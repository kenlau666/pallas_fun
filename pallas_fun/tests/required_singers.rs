use pallas_fun::wrapper::RequiredSignersWrapper;

#[test]
fn test_required_signers_wrapper_encode_decode() {
    let keyhashes = vec![
        "276fd18711931e2c0e21430192dbeac0e458093cd9d1fcd7210f64b3", // 56
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", // 56
        "1234567890abcdef1234567890abcdef1234567890aef123456890ab", // 56
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeaddead", // 56
        "00112233445566778899aabbccddeeff002233445566778899bbccdd", // 56
    ];
    let wrapper = RequiredSignersWrapper::new(keyhashes).expect("should create wrapper");

    let encoded = wrapper.encode();
    let decoded = RequiredSignersWrapper::decode(encoded).expect("should decode");

    assert_eq!(wrapper, decoded);
}

#[test]
fn test_required_signers_wrapper_invalid_keyhash() {
    // Use an invalid keyhash string
    let keyhash_str = "invalid";
    let result = RequiredSignersWrapper::new(vec![keyhash_str]);
    assert!(result.is_err());
}
