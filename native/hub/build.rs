use glob::glob;
use std::path::PathBuf;

fn main() {
    // Compile `.proto` files into Rust code.
    let protoc_path = protobuf_src::protoc();
    std::env::set_var("PROTOC", protoc_path);
    let protos_pattern = "../../messages/**/*.proto";
    let protos_paths: Vec<PathBuf> = glob(protos_pattern)
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    prost_build::Config::new()
        .out_dir("./src/messages")
        .compile_protos(&protos_paths, &["../../messages"])
        .unwrap();
}
