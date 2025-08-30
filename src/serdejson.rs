use serde :: {Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Person {
  name : String,
  age : u32
}

pub fn serialise_deserialize_json(){

  let person : Person = Person {
    name : String :: from ("Akhil"),
    age : 21
  };

  match serde_json :: to_string (&person) {
    Ok (json_str) => {
       println!("Serialized Data : {}", json_str);

       match serde_json :: from_str :: <Person> (&json_str) {
        Ok (deserialized_person) => {
           println!("Deserialized Data : {:?}", deserialized_person);
        } Err (e) => {
          eprintln!("Failed to Deserialize : {}",e)
        }
       }

    } Err (e) => {
      eprintln!("Failed to Serialize : {}",e)
    }
  }

}