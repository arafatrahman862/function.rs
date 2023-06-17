use databuf::Encode;

#[test]
fn test_name() {
    println!("{:?}", '💩'.to_bytes::<0>());
    println!(
        "{:?}",
        u32::from_le_bytes('💩'.to_bytes::<0>().try_into().unwrap())
    );
}
