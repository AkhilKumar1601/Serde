mod serdejson;
mod serdeyaml;
mod borsh;
fn main () {
  serdejson :: serialise_deserialize_json();
  serdeyaml :: serialize_deserialize_yaml();
  borsh :: serialize_deserialize_borsh();
}
