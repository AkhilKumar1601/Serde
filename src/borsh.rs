use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>,
}

pub fn serialize_deserialize_borsh() {
    let original = MyStruct {
        id: 42,
        data: "Hello, Borsh!".into(),
        v: vec![1, 2, 3],
    };

    let mut buffer: Vec<u8> = Vec::new();

    match original.serialize(&mut buffer) {
        Ok(_) => println!("Serialization successful."),
        Err(e) => {
            eprintln!("Serialization failed: {}", e);
            return;
        }
    }

    match MyStruct::try_from_slice(&buffer) {
        Ok(deserialized) => {
            assert_eq!(original, deserialized);
            println!("Successfully serialized and deserialized: {:?}", deserialized);
        }
        Err(e) => {
            eprintln!("Deserialization failed: {}", e);
        }
    }
}
