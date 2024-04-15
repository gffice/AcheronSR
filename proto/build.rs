use std::path::Path;

pub fn main() {
    let proto_file = "StarRail.proto";
    if Path::new(proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");

        prost_build::Config::new()
            .out_dir("out/")
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .compile_protos(&[proto_file], &["."])
            .unwrap();
    }
}
