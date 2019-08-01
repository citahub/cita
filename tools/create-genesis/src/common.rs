pub fn string_2_bytes(value: String) -> Vec<u8> {
    let v = Box::leak(value.into_boxed_str());
    let v = cita_types::clean_0x(v);
    hex::decode(v).unwrap()
}
