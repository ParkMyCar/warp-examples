use std::env;

fn main () {
    #[allow(non_snake_case)]
    let GEN_PROTO_DST: &str = "src/gen/";
    env::set_var("OUT_DIR", GEN_PROTO_DST);
    
    prost_build::compile_protos(&["proto/query.proto"], &["proto/"]).unwrap();
}