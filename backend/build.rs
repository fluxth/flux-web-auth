fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut protos: Vec<String> = vec![];
    for file in std::fs::read_dir("../proto/").unwrap() {
        let filepath = file.unwrap().path().to_str().unwrap().to_string();
        println!("cargo:rerun-if-changed={}", &filepath);
        protos.push(filepath);
    }

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&protos, &["../proto/"])
        .unwrap();
}
