use serde :: {Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct User {
  name : String,
  age : u32
}

#[derive(Debug,Serialize,Deserialize)]
struct MyStruct {
  id : u64,
  data : String,
  v : Vec<u32>,
  user : User
}

pub fn serialize_deserialize_yaml () {
  let s : MyStruct = MyStruct {
    id : 1,
    data : String :: from ("Akhil Kumar"),
    v : vec![13,32,4],
    user : User {
      name : String :: from ("Akhil"),
      age : 21
    }
  };

  let mut yaml_str = String :: new ();
  
  match serde_yaml :: to_string (&s) {
    Ok (yaml_str1) => {
      println!("Serialized Data : {}", yaml_str1);
      yaml_str = yaml_str1;
    } Err (e) => {
      eprintln!("Failed to Serialize : {}",e)
    }
  }

  match serde_yaml :: from_str :: <MyStruct> (&yaml_str) {
    Ok (deserialized_struct) => {
      println!("Deserialized Data : {:?}", deserialized_struct);
    } Err (e) => {
      eprintln!("Failed to Deserialize: {}", e);
    }
  }
}