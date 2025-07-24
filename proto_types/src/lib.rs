pub mod protovalidate {
  include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
}

pub mod protobuf {
  include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
}
