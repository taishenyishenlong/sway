experimental_storage_vec_testgen!(
    test_str_vec,
    "test_artifacts/storage_vec/svec_str/out/debug/svec_str-abi.json",
    "str",
    ::fuels::types::SizedAsciiString::<4>,
    ::fuels::types::SizedAsciiString::new("yeet".to_string()).unwrap(),
    ::fuels::types::SizedAsciiString::new("meow".to_string()).unwrap(),
    ::fuels::types::SizedAsciiString::new("kekw".to_string()).unwrap(),
    ::fuels::types::SizedAsciiString::new("gmgn".to_string()).unwrap(),
    ::fuels::types::SizedAsciiString::new("sway".to_string()).unwrap()
);
