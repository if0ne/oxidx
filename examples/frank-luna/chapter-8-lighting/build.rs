fn main() {
    println!("!cargo:rerun-if-changed=src/shader.hlsl");
    std::fs::copy(
        "src/shader.hlsl",
        std::env::var("OUT_DIR").unwrap() + "/../../../shader.hlsl",
    )
    .expect("Copy");

    println!("!cargo:rerun-if-changed=src/light_utils.hlsl");
    std::fs::copy(
        "src/light_utils.hlsl",
        std::env::var("OUT_DIR").unwrap() + "/../../../light_utils.hlsl",
    )
    .expect("Copy");
}
