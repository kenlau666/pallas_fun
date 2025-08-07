// use pallas_fun::{utils::IntoInner, wrapper::TransactionInputWrapper};

// #[test]
// fn test_transaction_input_wrapper_encode_decode() {
//     // Example 32-byte hex string for transaction id
//     let tx_id = "aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899";
//     let index = 3u64;

//     // Create new wrapper
//     let wrapper = TransactionInputWrapper::new(tx_id, index).expect("should create wrapper");

//     // Encode to hex
//     let encoded = wrapper.encode();

//     // Decode back to object
//     let decoded = TransactionInputWrapper::decode(encoded).expect("should decode");

//     // Check equality
//     assert_eq!(wrapper.into_inner(), decoded.into_inner());
// }
