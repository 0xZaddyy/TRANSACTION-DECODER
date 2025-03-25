use std::io::Read;

fn read_version( mut transaction_bytes: &[u8]) -> u32 {
    let  mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
   
    
}

fn main() {
    let transaction_hex = "010000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2c030ecb3d00040408dd6704c754a0160cb8f3a4675a250200000000000a636b706f6f6c082f636b706f6f6c2fffffffff026f11050000000000160014075cdd8fb868692bb05bae69050aa10d564acda00000000000000000266a24aa21a9ed854f256d625cacd48f6555140c939ba4f8ca48cfba98e29905f6922c96b633f20120000000000000000000000000000000000000000000000000000000000000000000000000";
    let transacton_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transacton_bytes.as_slice();
    let version = read_version(bytes_slice);

    let mut input_count =  [0; 1];
    bytes_slice.read(&mut input_count).unwrap();
    assert_eq!(input_count[0], 2_u8);



    println!("version: {}",  version);
}
