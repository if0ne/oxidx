fn main() {
    println!("!cargo:rerun-if-changed=src/shader.hlsl");
    std::fs::copy(
        "src/shader.hlsl",
        std::env::var("OUT_DIR").unwrap() + "/../../../shader.hlsl",
    )
    .expect("Copy");
}
