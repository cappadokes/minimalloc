fn main() {
    cxx_build::bridge("src/lib.rs")
        .compile("rustimer");

    println!("cargo:rerun-if-changed=src/lib.rs");
}