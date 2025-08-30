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

  let mut json_str = String :: new ();

  match serde_json :: to_string (&person) {
    Ok (json_str1) => {
       println!("Serialized Data : {}", json_str1);
       json_str = json_str1;
    } Err (e) => {
      eprintln!("Failed to Serialize : {}",e)
    }
  }

  match serde_json :: from_str :: <Person> (&json_str) {
    Ok (deserialized_person) => {
       println!("Deserialized Data : {:?}", deserialized_person);
    } Err (e) => {
      eprintln!("Failed to Deserialize : {}",e)
    }
   }


}