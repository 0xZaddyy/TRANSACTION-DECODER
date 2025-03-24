fn read_version(transaction_hex: &str) -> u32 {
    let transacton_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = <[u8; 4]>::try_from(&transacton_bytes[0..4]).unwrap();
    let _ = u32::from_le_bytes(version_bytes);
    

    1
}

fn main() {
    let version = read_version("010000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2c030ecb3d00040408dd6704c754a0160cb8f3a4675a250200000000000a636b706f6f6c082f636b706f6f6c2fffffffff026f11050000000000160014075cdd8fb868692bb05bae69050aa10d564acda00000000000000000266a24aa21a9ed854f256d625cacd48f6555140c939ba4f8ca48cfba98e29905f6922c96b633f20120000000000000000000000000000000000000000000000000000000000000000000000000");
    println!("version: {}",  version);
}
