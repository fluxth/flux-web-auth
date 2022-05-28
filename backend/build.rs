fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    for file in std::fs::read_dir("../proto/").unwrap() {
        let filepath = file.unwrap().path().to_str().unwrap().to_string();
        tonic_build::compile_protos(filepath).unwrap();
    }
}
