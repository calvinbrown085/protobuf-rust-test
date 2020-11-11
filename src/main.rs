use protobuf_codegen_pure::Codegen;
use crate::search_request::SearchRequest;
use crate::person::*;

mod search_request;
mod person;
fn main() {
    Codegen::new()
        .out_dir("src/")
        .inputs(&["protos/search_request.proto", "protos/person.proto"])
        .include("protos")
        .run()
        .expect("protoc");

}